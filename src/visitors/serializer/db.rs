use mysql::*;
use mysql::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct MysqlSerializer {
    pool: Arc<Pool>,
}

impl MysqlSerializer {
    pub fn new(database_url: &str) -> Self {
        let pool = Pool::new(database_url).expect("Failed to create MySQL connection pool");
        MysqlSerializer { pool: Arc::new(pool) }
    }

    fn get_or_create_player(&self, name: &str) -> u64 {
        let mut conn = self.pool.get_conn().unwrap();
        if let Ok(Some(id)) = conn.query_first::<u64, _>(
            "SELECT PlayerId FROM Player WHERE Name = ?", 
            (name,)
        ) {
            return id;
        }
        conn.exec_drop("INSERT INTO Player (Name) VALUES (?)", (name,)).unwrap();
        conn.last_insert_id()
    }

    fn get_or_create_opening(&self, name: &str, eco_code: &str) -> u64 {
        let mut conn = self.pool.get_conn().unwrap();
        if let Ok(Some(id)) = conn.query_first::<u64, _>(
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

    fn insert_game(&self, game: &Game) {
        let mut conn = self.pool.get_conn().unwrap();

        let white_id = self.get_or_create_player(&game.white);
        let black_id = self.get_or_create_player(&game.black);
        let opening_id = self.get_or_create_opening(&game.opening, &game.eco);

        conn.exec_drop(
            r#"INSERT INTO Game (
                RuleSetId, OpeningId, FCId,
                White, WhiteElo, WhiteTitle,
                Black, BlackElo, BlackTitle,
                Result, Termination, DateTime,
                HasClock, HasEvaluations
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            (
                1, opening_id, 1,
                white_id, &game.white_elo, null_if_empty(&game.white_title),
                black_id, &game.black_elo, null_if_empty(&game.black_title),
                map_result(&game.result), map_termination(&game.termination),
                format!("{} {}", game.utc_date, game.utc_time),
                0, 0
            )
        ).unwrap();
    }
}

fn null_if_empty(s: &str) -> Option<&str> {
    if s.trim().is_empty() || s == "?" { None } else { Some(s) }
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
