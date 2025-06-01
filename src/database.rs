//! Module that handles all database operations, from connecting to it, creating the database and its tables and inserting the data.

#[cfg(all(
    any(feature = "time-mysql", feature = "chrono-mysql"),
    any(feature = "time-diesel", feature = "chrono-diesel")
))]
compile_error!("The features of mysql and of diesel cannot be enabled at the same time.");

#[cfg(not(any(
    feature = "time-mysql",
    feature = "chrono-mysql",
    feature = "time-diesel",
    feature = "chrono-diesel"
)))]
compile_error!("At least one of the features of mysql or of diesel must be enabled.");

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
pub use mysql::*;

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
mod mysql {
    use log::{info, trace};

    use mysql::{Conn, Error, OptsBuilder, prelude::Queryable};

    use lichess::{
        attributes::{BoardConfiguration, Eco, Opening, Player, RuleSet},
        data::{Data, Game, Move},
    };

    /// Gets a connection to MySQL.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the connection fails to start.
    pub fn get_conn(db_password: &str) -> Result<Conn, Error> {
        Conn::new(
            OptsBuilder::new()
                .user(Some("root"))
                .pass(Some(db_password)),
        )
    }

    /// Creates and selects the lichess database.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the creation or selection fails.
    pub fn create_db(conn: &mut Conn) -> Result<(), Error> {
        conn.query_drop(include_str!("sql/create-database.sql"))?;
        conn.select_db("lichess")
    }

    /// Creates the FinalConfiguration table of the database.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the creation fails.
    pub fn create_final_configuration(conn: &mut Conn) -> Result<(), Error> {
        trace!("create_final_configuration function.");
        info!("Creating FinalConfiguration table.");
        conn.query_drop(include_str!("sql/create-finalconf.sql"))
    }

    /// Creates the Game table of the database.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the creation fails.
    pub fn create_game(conn: &mut Conn) -> Result<(), Error> {
        trace!("create_game function.");
        info!("Creating Game table.");
        conn.query_drop(include_str!("sql/create-game.sql"))
    }

    /// Creates the Move table of the database.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the creation fails.
    pub fn create_move(conn: &mut Conn) -> Result<(), Error> {
        trace!("create_move function.");
        info!("Creating Move table.");
        conn.query_drop(include_str!("sql/create-move.sql"))
    }

    /// Creates the Opening table of the database.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the creation fails.
    pub fn create_opening(conn: &mut Conn) -> Result<(), Error> {
        trace!("create_opening function.");
        info!("Creating Opening table.");
        conn.query_drop(include_str!("sql/create-opening.sql"))
    }

    /// Creates the Player table of the database.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the creation fails.
    pub fn create_player(conn: &mut Conn) -> Result<(), Error> {
        trace!("create_player function.");
        info!("Creating Player table.");
        conn.query_drop(include_str!("sql/create-player.sql"))
    }

    /// Creates the RuleSet table of the database.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the creation fails.
    pub fn create_ruleset(conn: &mut Conn) -> Result<(), Error> {
        trace!("create_ruleset function.");
        info!("Creating RuleSet table.");
        conn.query_drop(include_str!("sql/create-ruleset.sql"))
    }

    /// Creates the lichess database, selects it and creates all its tables.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the creation or selection fails.
    pub fn create_full_database(conn: &mut Conn) -> Result<(), Error> {
        trace!("create_full_database function.");
        info!("Creating full database.");
        create_db(conn)?;
        create_final_configuration(conn)?;
        create_opening(conn)?;
        create_player(conn)?;
        create_ruleset(conn)?;
        create_game(conn)?;
        create_move(conn)?;
        info!("Database created correctly.");
        Ok(())
    }

    /// Gets a connection to MySQL, creates the lichess database, selects it and creates all its tables.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the connection fails to start or the creation or selection fails.
    #[allow(dead_code)]
    pub fn initialize_database(db_password: &str) -> Result<Conn, Error> {
        trace!("initialize_database function.");
        info!("Initializing connection.");
        let mut conn = get_conn(db_password)?;
        create_full_database(&mut conn)?;
        Ok(conn)
    }

    /// Gets a connection to MySQL, creates the lichess database and all its tables if it doesn't exist and selects it.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the connection fails to start or the creation or selection fails.
    pub fn initialize_database_if_not_exists(db_password: &str) -> Result<Conn, Error> {
        trace!("initialize_database_if_not_exists function.");
        info!("Initializing connection.");
        let mut conn = get_conn(db_password)?;
        if conn.select_db("lichess").is_ok() {
            info!("Database already exists, proceeding.");
            return Ok(conn);
        }
        info!("Database doesn't exist, creating.");
        create_full_database(&mut conn)?;
        Ok(conn)
    }

    /// Inserts a [`BoardConfiguration`] into the FinalConfiguration table.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the insertion fails.
    pub fn insert_final_configuration(
        conn: &mut Conn,
        final_configuration: &BoardConfiguration,
    ) -> Result<u64, Error> {
        let params = final_configuration.as_params();
        match conn.exec_iter(include_str!("sql/insert-finalconf.sql"), &params) {
            Ok(result) => {
                return Ok(result
                .last_insert_id()
                .expect("The query is a final configuration insertion query and thus must return an insert Id."));
            }
            Err(e) => match e {
                Error::MySqlError(mut mse) => {
                    if !mse.message.starts_with("Duplicate entry") {
                        mse.message
                            .push_str(&format!(" (FinalConfiguration: {:?})", final_configuration));
                        return Err(Error::MySqlError(mse));
                    }
                }
                _ => return Err(e),
            },
        }
        Ok(conn
        .exec_first::<u64, _, _>(include_str!("sql/select-finalconf.sql"), params)?
        .expect("There was an error when trying to insert a final configuration so it must have existed already."))
    }

    /// Inserts a [`Opening`] into the Opening table.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the insertion fails.
    pub fn insert_opening(conn: &mut Conn, opening: &Opening, eco: Eco) -> Result<u64, Error> {
        match conn.exec_iter(
            include_str!("sql/insert-opening.sql"),
            opening.as_insert_params(eco),
        ) {
            Ok(result) => {
                return Ok(result.last_insert_id().expect(
                    "The query is an opening insertion query and thus must return an insert id.",
                ));
            }
            Err(e) => match e {
                Error::MySqlError(mut mse) => {
                    if !mse.message.starts_with("Duplicate entry") {
                        mse.message.push_str(" (Opening: ");
                        mse.message.push_str(&opening.0);
                        mse.message.push(' ');
                        mse.message.push(eco.0.as_char());
                        mse.message.push_str(&format!("{:02})", eco.1.get()));
                        return Err(Error::MySqlError(mse));
                    }
                }
                _ => return Err(e),
            },
        }
        Ok(conn
        .exec_first::<u64, _, _>(
            include_str!("sql/select-opening.sql"),
            opening.as_select_params(),
        )?
        .expect(
            "There was an error when trying to insert an opening so it must have existed already.",
        ))
    }

    /// Inserts a [`Player`] into the Player table.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the insertion fails.
    pub fn insert_player(conn: &mut Conn, player: &Player) -> Result<u64, Error> {
        let params = player.as_params();
        match conn.exec_iter(include_str!("sql/insert-player.sql"), &params) {
            Ok(result) => {
                return Ok(result.last_insert_id().expect(
                    "The query is a player insertion query and thus must return an insert id.",
                ));
            }
            Err(e) => match e {
                Error::MySqlError(mut mse) => {
                    if !mse.message.starts_with("Duplicate entry") {
                        mse.message.push_str(" (Player: ");
                        mse.message.push_str(&player.0);
                        mse.message.push(')');
                        return Err(Error::MySqlError(mse));
                    }
                }
                _ => return Err(e),
            },
        }
        Ok(conn
        .exec_first::<u64, _, _>(include_str!("sql/select-player.sql"), params)?
        .expect(
            "There was an error when trying to insert a player so it must have existed already.",
        ))
    }

    /// Inserts a [`RuleSet`] into the RuleSet table.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the insertion fails.
    pub fn insert_ruleset(conn: &mut Conn, ruleset: &RuleSet) -> Result<u64, Error> {
        match conn.exec_iter(
            include_str!("sql/insert-ruleset.sql"),
            ruleset.as_insert_params(),
        ) {
            Ok(result) => {
                return Ok(result.last_insert_id().expect(
                    "The query is a ruleset insertion query and thus must return an insert id.",
                ));
            }
            Err(e) => match e {
                Error::MySqlError(mut mse) => {
                    if !mse.message.starts_with("Duplicate entry") {
                        mse.message.push_str(" (RuleSet: ");
                        mse.message.push_str(&ruleset.name);
                        mse.message.push_str(" - ");
                        mse.message.push_str(ruleset.kind.as_str());
                        mse.message.push(')');
                        return Err(Error::MySqlError(mse));
                    }
                }
                _ => return Err(e),
            },
        }
        Ok(conn
        .exec_first::<u64, _, _>(
            include_str!("sql/select-ruleset.sql"),
            ruleset.as_select_params(),
        )?
        .expect(
            "There was an error when trying to insert a ruleset so it must have existed already.",
        ))
    }

    /// Inserts a [`Game`] into the Game table.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the insertion fails.
    pub fn insert_game(
        conn: &mut Conn,
        game: &Game,
        ruleset_id: u64,
        opening_id: Option<u64>,
        fc_id: u64,
        white_id: Option<u64>,
        black_id: Option<u64>,
    ) -> Result<u64, Error> {
        Ok(conn
            .exec_iter(
                include_str!("sql/insert-game.sql"),
                game.as_params(ruleset_id, opening_id, fc_id, white_id, black_id),
            )?
            .last_insert_id()
            .expect("The query is a game insertion query and thus must return an insert id."))
    }

    /// Inserts a [`Game`]'s data into the Game, RuleSet, Opening, FinalConfiguration and Player tables.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if any of the insertions fail.
    pub fn insert_game_data(conn: &mut Conn, game: &Game) -> Result<u64, Error> {
        let ruleset_id = insert_ruleset(conn, &game.ruleset)?;
        let opening_id = if game.opening.0.is_empty() {
            None
        } else {
            Some(insert_opening(conn, &game.opening, game.eco)?)
        };
        let fc_id = insert_final_configuration(conn, &game.final_conf)?;
        let white_id = if game.white.0.is_empty() {
            None
        } else {
            Some(insert_player(conn, &game.white)?)
        };
        let black_id = if game.black.0.is_empty() {
            None
        } else {
            Some(insert_player(conn, &game.black)?)
        };
        insert_game(
            conn, game, ruleset_id, opening_id, fc_id, white_id, black_id,
        )
    }

    /// Inserts a [`Move`] into the Move table.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the insertion fails.
    #[allow(dead_code)]
    pub fn insert_move(conn: &mut Conn, r#move: &Move, game_id: u64) -> Result<(), Error> {
        conn.exec_drop(
            include_str!("sql/insert-move.sql"),
            r#move.as_params(game_id),
        )
    }

    /// Inserts a [`Vec`] of [`Move`]s into the Move table.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if the insertion fails.
    pub fn insert_moves(conn: &mut Conn, moves: &Vec<Move>, game_id: u64) -> Result<(), Error> {
        conn.exec_batch(
            include_str!("sql/insert-move.sql"),
            moves.iter().map(|r#move| r#move.as_params(game_id)),
        )
    }

    /// Inserts a [`Data`]'s [`Game`] and [`Vec`] of [`Move`]s into the Game, RuleSet, Opening, FinalConfiguration, Player and Move tables.
    ///
    /// # Errors
    /// Will return [`mysql::Error`] if any of the insertions fail.
    pub fn insert_all(conn: &mut Conn, data: &Data) -> Result<(), Error> {
        let game_id = insert_game_data(conn, &data.game)?;
        insert_moves(conn, &data.moves, game_id)
    }
}
