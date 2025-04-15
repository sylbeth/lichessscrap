use lichess::attributes::RuleSet;
use mysql::*;
use mysql::prelude::*;
use std::sync::Arc;
use std::fmt;

#[cfg(feature = "mysql")]
use lichess::data::{game::Game, r#move::Move};

#[cfg(feature = "mysql")]
pub struct DbSerializer {
    pool: Arc<Pool>,
    max_games: usize,
    game_ids: Vec<(usize, u64)>,
}

impl fmt::Debug for DbSerializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DbSerializer")
            .finish()
    }
}

#[cfg(feature = "mysql")]
impl DbSerializer {
    pub fn new(database_url: &str) -> Self {
        let pool = Pool::new(database_url).expect("Failed to create MySQL connection pool");
        DbSerializer {pool: Arc::new(pool) ,
                        max_games:100,
                        game_ids: Vec::new()}

        }

    pub fn get_or_create_player(&self, name: &str) -> u64 {
        let mut conn = self.pool.get_conn().unwrap();
        if let Ok(Some(id)) = conn.exec_first::<u64, _, _>(
            "SELECT PlayerId FROM Player WHERE Name = ?", 
            (name,)
        ) {
            return id;
        }
        conn.exec_drop("INSERT INTO Player (Name) VALUES (?)", (name,)).unwrap();
        conn.last_insert_id()
    }

    pub fn get_or_create_opening(&self, name: &str, eco_code: &str) -> u64 {
        let mut conn = self.pool.get_conn().unwrap();
        if let Ok(Some(id)) = conn.exec_first::<u64, _, _>(
            "SELECT OpeningId FROM Opening WHERE Name = ?", 
            (name,)
        ) {
            return id;
        }

        let (eco_letter, eco_number) = if eco_code == "?" || eco_code.is_empty() {
            (None, 0)
        } else {
            let (letter, number) = eco_code.split_at(1);
            let num = number.parse::<u8>().unwrap_or(0);
            (Some(letter.to_string()), num)
        };

        conn.exec_drop(
            "INSERT INTO Opening (Name, EcoLetter, EcoNumber) VALUES (?, ?, ?)",
            (&name, &eco_letter, &eco_number)
        ).unwrap();

        conn.last_insert_id()
    }

    pub fn get_or_create_rule_set(&self, event: &str) -> u64 {
        let mut conn = self.pool.get_conn().unwrap();
        if let Ok(Some(id)) = conn.exec_first::<u64, _, _>(
            "SELECT RuleSetId FROM RuleSet WHERE Name = ?", 
            (event,)
        ) {
            return id;
        }
        conn.exec_drop("INSERT INTO RuleSet (Name) VALUES (?)", (event,)).unwrap();
        conn.last_insert_id()
    }

    pub fn get_or_create_final_position(&self, position: &str) -> u64 {
        let mut conn = self.pool.get_conn().unwrap();
        if let Ok(Some(id)) = conn.exec_first::<u64, _, _>(
            "SELECT FinalPositionId FROM FinalPosition WHERE  Position = ?", 
            (position)
        ) {
            return id;
        }
        conn.exec_drop(
            "INSERT INTO FinalPosition (Position) VALUES (?)",
            (position,)
        ).unwrap();
        conn.last_insert_id()
    }


    pub fn write_game(&mut self, game: &Game) {
        if self.game_ids.len() >= self.max_games {
            return; 
        }
        let mut conn = self.pool.get_conn().unwrap();
        
        let ruleSetId = self.get_or_create_rule_set(&game.event);

        let white_id = self.get_or_create_player(&game.white);
        let black_id = self.get_or_create_player(&game.black);
        let opening_id = self.get_or_create_opening(&game.opening, &game.eco);

        conn.exec_drop(
            r#"INSERT INTO Game (
                RuleSetId, OpeningId, FCId,
                White, WhiteElo, WhiteTitle,
                Black, BlackElo, BlackTitle,
                Result, Termination, DateTime,
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            (
                ruleSetId, opening_id, 1,
                white_id, &game.white_elo, &game.white_title,
                black_id, &game.black_elo, &game.black_title,
                map_result(&game.result), &game.termination,
                format!("{} {}", game.utc_date, game.utc_time),
            )
        ).unwrap();//Ya añadiremos el resto de parametros más tarde
        
        self.game_ids.push((game.game_id.clone(), conn.last_insert_id()));
        
        
    }
    pub fn write_move(&self, r#move: &Move) {
        let mut conn = self.pool.get_conn().unwrap();
        conn.exec_drop(
            r#"INSERT INTO Move (
                GameId, Num, San, Nag, Eval, Clk
            ) VALUES (?, ?, ?, ?, ?, ?)"#,
            (
                0, r#move.num,
                r#move.san.clone(), r#move.nag.clone(),
                null_if_empty(&r#move.eval), null_if_empty(&r#move.clk)
            )
        ).unwrap();
    }
}

fn null_if_empty(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn map_result(result: &str) -> Option<u8> {
    match result {
        "1-0" => Some(1),
        "0-1" => Some(2),
        "1/2-1/2" => Some(3),
        _ => None,
    }
}

fn map_termination(term: &str) -> Option<u8> {
    match term {
        "Normal" => Some(1),
        "Abandonment" => Some(2),
        "Time forfeit" => Some(3),
        "Stalemate" => Some(4),
        "Checkmate" => Some(5),
        _ => None,
    }
}
