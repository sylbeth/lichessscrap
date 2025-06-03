use log::{info, trace};

use mysql::{Conn, prelude::Queryable};

use lichess::{
    attributes::{BoardConfiguration, Eco, Opening, Player, RuleSet},
    data::{Data, Game, Move},
};

use super::{Connection, DatabaseAdapter};

impl DatabaseAdapter for Connection {
    type Error = mysql::Error;

    fn new(db_url: &str) -> Result<Self, Self::Error> {
        trace!("Connection new function.");
        info!("Connecting to the database.");
        Ok(Self {
            conn: Conn::new(db_url)?,
        })
    }

    fn create_db(&mut self) -> Result<&mut Self, Self::Error> {
        trace!("Connection create_db function.");
        info!("Creating the database.");
        self.conn
            .query_drop(include_str!("../sql/create-database.sql"))?;
        self.conn.select_db("lichess")?;
        Ok(self)
    }

    fn create_final_configuration(&mut self) -> Result<&mut Self, Self::Error> {
        trace!("Connection create_final_configuration function.");
        info!("Creating FinalConfiguration table.");
        self.conn
            .query_drop(include_str!("../sql/create-finalconf.sql"))?;
        Ok(self)
    }

    fn create_game(&mut self) -> Result<&mut Self, Self::Error> {
        trace!("Connection create_game function.");
        info!("Creating Game table.");
        self.conn
            .query_drop(include_str!("../sql/create-game.sql"))?;
        Ok(self)
    }

    fn create_move(&mut self) -> Result<&mut Self, Self::Error> {
        trace!("Connection create_move function.");
        info!("Creating Move table.");
        self.conn
            .query_drop(include_str!("../sql/create-move.sql"))?;
        Ok(self)
    }

    fn create_opening(&mut self) -> Result<&mut Self, Self::Error> {
        trace!("Connection create_opening function.");
        info!("Creating Opening table.");
        self.conn
            .query_drop(include_str!("../sql/create-opening.sql"))?;
        Ok(self)
    }

    fn create_player(&mut self) -> Result<&mut Self, Self::Error> {
        trace!("Connection create_player function.");
        info!("Creating Player table.");
        self.conn
            .query_drop(include_str!("../sql/create-player.sql"))?;
        Ok(self)
    }

    fn create_ruleset(&mut self) -> Result<&mut Self, Self::Error> {
        trace!("Connection create_ruleset function.");
        info!("Creating RuleSet table.");
        self.conn
            .query_drop(include_str!("../sql/create-ruleset.sql"))?;
        Ok(self)
    }

    fn create_views(&mut self) -> Result<&mut Self, Self::Error> {
        trace!("Connection create_views function.");
        info!("Creating views.");
        self.conn
            .query_drop(include_str!("../sql/view-descriptor.sql"))?;
        self.conn
            .query_drop(include_str!("../sql/view-finalboard.sql"))?;
        self.conn
            .query_drop(include_str!("../sql/view-piecesleft.sql"))?;
        Ok(self)
    }

    fn create_full_database(&mut self) -> Result<&mut Self, Self::Error> {
        trace!("Connection create_full_database function.");
        info!("Creating full database.");
        self.create_db()?
            .create_final_configuration()?
            .create_opening()?
            .create_player()?
            .create_ruleset()?
            .create_game()?
            .create_move()?
            .create_views()?;
        info!("Database created correctly.");
        Ok(self)
    }

    fn initialize_database(db_url: &str, rebuild: bool) -> Result<Self, Self::Error> {
        trace!("Connection initialize_database function.");
        info!("Initializing connection.");
        let mut connection = Self::new(db_url)?;
        if rebuild {
            info!("Rebuilding database.");
        } else if connection.conn.select_db("lichess").is_ok() {
            info!("Database already exists, proceeding.");
            return Ok(connection);
        } else {
            info!("Database doesn't exist, creating.");
        }
        connection.create_full_database()?;
        Ok(connection)
    }

    fn insert_final_configuration(
        &mut self,
        final_configuration: &BoardConfiguration,
    ) -> Result<u64, Self::Error> {
        let params = final_configuration.as_params();
        match self
            .conn
            .exec_iter(include_str!("../sql/insert-finalconf.sql"), &params)
        {
            Ok(result) => {
                return Ok(result
                .last_insert_id()
                .expect("The query is a final configuration insertion query and thus must return an insert Id."));
            }
            Err(e) => match e {
                Self::Error::MySqlError(mut mse) => {
                    if !mse.message.starts_with("Duplicate entry") {
                        mse.message
                            .push_str(&format!(" (FinalConfiguration: {:?})", final_configuration));
                        return Err(Self::Error::MySqlError(mse));
                    }
                }
                _ => return Err(e),
            },
        }
        Ok(self.conn
        .exec_first::<u64, _, _>(include_str!("../sql/select-finalconf.sql"), params)?
        .expect("There was an error when trying to insert a final configuration so it must have existed already."))
    }

    fn insert_opening(&mut self, opening: &Opening, eco: Eco) -> Result<u64, Self::Error> {
        match self.conn.exec_iter(
            include_str!("../sql/insert-opening.sql"),
            opening.as_insert_params(eco),
        ) {
            Ok(result) => {
                return Ok(result.last_insert_id().expect(
                    "The query is an opening insertion query and thus must return an insert id.",
                ));
            }
            Err(e) => match e {
                Self::Error::MySqlError(mut mse) => {
                    if !mse.message.starts_with("Duplicate entry") {
                        mse.message.push_str(" (Opening: ");
                        mse.message.push_str(&opening.0);
                        mse.message.push(' ');
                        mse.message.push(eco.0.as_char());
                        mse.message.push_str(&format!("{:02})", eco.1.get()));
                        return Err(Self::Error::MySqlError(mse));
                    }
                }
                _ => return Err(e),
            },
        }
        Ok(self.conn
        .exec_first::<u64, _, _>(
            include_str!("../sql/select-opening.sql"),
            opening.as_select_params(),
        )?
        .expect(
            "There was an error when trying to insert an opening so it must have existed already.",
        ))
    }

    fn insert_player(&mut self, player: &Player) -> Result<u64, Self::Error> {
        let params = player.as_params();
        match self
            .conn
            .exec_iter(include_str!("../sql/insert-player.sql"), &params)
        {
            Ok(result) => {
                return Ok(result.last_insert_id().expect(
                    "The query is a player insertion query and thus must return an insert id.",
                ));
            }
            Err(e) => match e {
                Self::Error::MySqlError(mut mse) => {
                    if !mse.message.starts_with("Duplicate entry") {
                        mse.message.push_str(" (Player: ");
                        mse.message.push_str(&player.0);
                        mse.message.push(')');
                        return Err(Self::Error::MySqlError(mse));
                    }
                }
                _ => return Err(e),
            },
        }
        Ok(self.conn
        .exec_first::<u64, _, _>(include_str!("../sql/select-player.sql"), params)?
        .expect(
            "There was an error when trying to insert a player so it must have existed already.",
        ))
    }

    fn insert_ruleset(&mut self, ruleset: &RuleSet) -> Result<u64, Self::Error> {
        match self.conn.exec_iter(
            include_str!("../sql/insert-ruleset.sql"),
            ruleset.as_insert_params(),
        ) {
            Ok(result) => {
                return Ok(result.last_insert_id().expect(
                    "The query is a ruleset insertion query and thus must return an insert id.",
                ));
            }
            Err(e) => match e {
                Self::Error::MySqlError(mut mse) => {
                    if !mse.message.starts_with("Duplicate entry") {
                        mse.message.push_str(" (RuleSet: ");
                        mse.message.push_str(&ruleset.name);
                        mse.message.push_str(" - ");
                        mse.message.push_str(ruleset.kind.as_str());
                        mse.message.push(')');
                        return Err(Self::Error::MySqlError(mse));
                    }
                }
                _ => return Err(e),
            },
        }
        Ok(self.conn
        .exec_first::<u64, _, _>(
            include_str!("../sql/select-ruleset.sql"),
            ruleset.as_select_params(),
        )?
        .expect(
            "There was an error when trying to insert a ruleset so it must have existed already.",
        ))
    }

    fn insert_game(
        &mut self,
        game: &Game,
        ruleset_id: u64,
        opening_id: Option<u64>,
        fc_id: u64,
        white_id: Option<u64>,
        black_id: Option<u64>,
    ) -> Result<u64, Self::Error> {
        Ok(self
            .conn
            .exec_iter(
                include_str!("../sql/insert-game.sql"),
                game.as_params(ruleset_id, opening_id, fc_id, white_id, black_id),
            )?
            .last_insert_id()
            .expect("The query is a game insertion query and thus must return an insert id."))
    }

    fn insert_game_data(&mut self, game: &Game) -> Result<u64, Self::Error> {
        let ruleset_id = self.insert_ruleset(&game.ruleset)?;
        let opening_id = if game.opening.0.is_empty() {
            None
        } else {
            Some(self.insert_opening(&game.opening, game.eco)?)
        };
        let fc_id = self.insert_final_configuration(&game.final_conf)?;
        let white_id = if game.white.0.is_empty() {
            None
        } else {
            Some(self.insert_player(&game.white)?)
        };
        let black_id = if game.black.0.is_empty() {
            None
        } else {
            Some(self.insert_player(&game.black)?)
        };
        self.insert_game(game, ruleset_id, opening_id, fc_id, white_id, black_id)
    }

    fn insert_move(&mut self, r#move: &Move, game_id: u64) -> Result<&mut Self, Self::Error> {
        self.conn.exec_drop(
            include_str!("../sql/insert-move.sql"),
            r#move.as_params(game_id),
        )?;
        Ok(self)
    }

    fn insert_moves(&mut self, moves: &[Move], game_id: u64) -> Result<&mut Self, Self::Error> {
        self.conn.exec_batch(
            include_str!("../sql/insert-move.sql"),
            moves.iter().map(|r#move| r#move.as_params(game_id)),
        )?;
        Ok(self)
    }

    fn insert_all(&mut self, data: &Data) -> Result<&mut Self, Self::Error> {
        let game_id = self.insert_game_data(&data.game)?;
        self.insert_moves(&data.moves, game_id)
    }
}