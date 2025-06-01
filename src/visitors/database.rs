//! A visitor that inserts the elements of a PGN file to the database.

use std::error::Error;

use log::{error, info};
use pgn_reader::{Nag, RawComment, RawHeader, SanPlus, Visitor};
use shakmaty::Outcome;

use crate::{
    database::{initialize_database_if_not_exists, insert_all},
    visitors::comment_iterator::CommentIterator,
};
use lichess::data::Data;

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
use mysql::Conn;

/// A visitor that inserts the elements of a PGN file to the database.
#[derive(Debug)]
pub struct Database {
    /// Connection to the database.
    #[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
    database_connection: Conn,
    /// Current data as it is being collected.
    pub data: Data,
    /// Whether there were or not errors in the insertion to the database.
    pub has_errors: bool,
}

impl Database {
    /// Creates a new database serializer from the password.
    pub fn new(db_url: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            database_connection: initialize_database_if_not_exists(db_url)?,
            data: Data::default(),
            has_errors: false,
        })
    }
}

impl Visitor for Database {
    type Result = ();

    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        self.data.process_header(_key, _value.0);
    }

    fn san(&mut self, _san: SanPlus) {
        self.data.new_move(_san);
    }

    fn nag(&mut self, _nag: Nag) {
        self.data.add_nag(_nag);
    }

    fn comment(&mut self, _comment: RawComment<'_>) {
        for (key, value) in CommentIterator::new(_comment.0) {
            self.data.process_comment(key, value);
        }
    }

    fn outcome(&mut self, _outcome: Option<Outcome>) {
        self.data.check_outcome(_outcome);
    }

    fn end_game(&mut self) {
        self.data.end_game();
        if let Err(e) = insert_all(&mut self.database_connection, &self.data) {
            error!("{} - Insertion error: {}", self.data.games, e);
            self.has_errors = true;
        }
        self.data.new_game();
        if self.data.games % 1000 == 0 {
            info!("Inserted data of {} games.", self.data.games);
        }
    }
}
