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

use lichess::{
    attributes::{BoardConfiguration, Eco, Opening, Player, RuleSet},
    data::{Data, Game, Move},
};

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
mod mysql;

#[cfg(any(feature = "time-diesel", feature = "chrono-diesel"))]
mod diesel;

/// An adapter for the MySQL database, holding a connection.
#[derive(Debug)]
pub struct Connection {
    /// The [`mysql`] [`Conn`](::mysql::Conn) to interact with the MySQL database.
    #[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
    conn: ::mysql::Conn,
}

pub trait DatabaseAdapter: Sized {
    type Error: std::error::Error;

    /// Constructs a connection to MySQL.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the connection fails to start.
    fn new(db_url: &str) -> Result<Self, Self::Error>;

    /// Creates and selects the lichess database.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the creation or selection fails.
    fn create_db(&mut self) -> Result<&mut Self, Self::Error>;

    /// Creates the FinalConfiguration table of the database.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the creation fails.
    fn create_final_configuration(&mut self) -> Result<&mut Self, Self::Error>;

    /// Creates the Game table of the database.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the creation fails.
    fn create_game(&mut self) -> Result<&mut Self, Self::Error>;

    /// Creates the Move table of the database.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the creation fails.
    fn create_move(&mut self) -> Result<&mut Self, Self::Error>;

    /// Creates the Opening table of the database.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the creation fails.
    fn create_opening(&mut self) -> Result<&mut Self, Self::Error>;

    /// Creates the Player table of the database.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the creation fails.
    fn create_player(&mut self) -> Result<&mut Self, Self::Error>;

    /// Creates the RuleSet table of the database.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the creation fails.
    fn create_ruleset(&mut self) -> Result<&mut Self, Self::Error>;

    /// Creates the views of the database: MoveDescriptor, PiecesLeft and FinalBoard.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if any of the creations fails.
    fn create_views(&mut self) -> Result<&mut Self, Self::Error>;

    /// Creates the lichess database, selects it and creates all its tables.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the creation or selection fails.
    fn create_full_database(&mut self) -> Result<&mut Self, Self::Error>;

    /// Gets a connection to MySQL, creates the lichess database, selects it and creates all its tables.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the connection fails to start or the creation or selection fails.
    fn initialize_database(db_url: &str) -> Result<Self, Self::Error>;

    /// Gets a connection to MySQL, creates the lichess database and all its tables if it doesn't exist and selects it.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the connection fails to start or the creation or selection fails.
    fn initialize_database_if_not_exists(db_url: &str) -> Result<Self, Self::Error>;

    /// Inserts a [`BoardConfiguration`] into the FinalConfiguration table.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the insertion fails.
    fn insert_final_configuration(
        &mut self,
        final_configuration: &BoardConfiguration,
    ) -> Result<u64, Self::Error>;

    /// Inserts a [`Opening`] into the Opening table.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the insertion fails.
    fn insert_opening(&mut self, opening: &Opening, eco: Eco) -> Result<u64, Self::Error>;

    /// Inserts a [`Player`] into the Player table.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the insertion fails.
    fn insert_player(&mut self, player: &Player) -> Result<u64, Self::Error>;

    /// Inserts a [`RuleSet`] into the RuleSet table.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the insertion fails.
    fn insert_ruleset(&mut self, ruleset: &RuleSet) -> Result<u64, Self::Error>;

    /// Inserts a [`Game`] into the Game table.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the insertion fails.
    fn insert_game(
        &mut self,
        game: &Game,
        ruleset_id: u64,
        opening_id: Option<u64>,
        fc_id: u64,
        white_id: Option<u64>,
        black_id: Option<u64>,
    ) -> Result<u64, Self::Error>;

    /// Inserts a [`Game`]'s data into the Game, RuleSet, Opening, FinalConfiguration and Player tables.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if any of the insertions fail.
    fn insert_game_data(&mut self, game: &Game) -> Result<u64, Self::Error>;

    /// Inserts a [`Move`] into the Move table.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the insertion fails.
    fn insert_move(&mut self, r#move: &Move, game_id: u64) -> Result<&mut Self, Self::Error>;

    /// Inserts a [`Vec`] of [`Move`]s into the Move table.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if the insertion fails.
    fn insert_moves(&mut self, moves: &[Move], game_id: u64) -> Result<&mut Self, Self::Error>;

    /// Inserts a [`Data`]'s [`Game`] and [`Vec`] of [`Move`]s into the Game, RuleSet, Opening, FinalConfiguration, Player and Move tables.
    ///
    /// # Errors
    /// Will return [`DatabaseAdapter::Error`] if any of the insertions fail.
    fn insert_all(&mut self, data: &Data) -> Result<&mut Self, Self::Error>;
}
