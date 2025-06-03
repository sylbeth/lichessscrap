//! A visitor that inserts the elements of a PGN file to the database.

use std::num::NonZeroUsize;

use log::info;
use pgn_reader::{Nag, RawComment, RawHeader, SanPlus, Visitor};
use shakmaty::Outcome;

use crate::{
    adapter::{Connection, DatabaseAdapter},
    visitors::comment_iterator::CommentIterator,
};
use lichess::data::Data;

/// A visitor that inserts the elements of a PGN file to the database.
#[derive(Debug)]
pub struct Database {
    /// Connection to the database.
    database_connection: Connection,
    /// Current data as it is being collected.
    pub data: Data,
}

impl Database {
    /// Creates a new database serializer from the password.
    pub fn new(
        db_url: &str,
        rebuild: bool,
        max_threads: NonZeroUsize,
    ) -> Result<Self, <Connection as DatabaseAdapter>::Error> {
        Ok(Self {
            database_connection: Connection::initialize_database(db_url, rebuild, max_threads)?,
            data: Data::default(),
        })
    }

    pub fn finish_insertion(self) -> () {
        self.database_connection.finish_insertion();
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
        self.database_connection.insert_all(&self.data);
        self.data.new_game();
        if self.data.games % 1000 == 0 {
            info!("Inserted data of {} games.", self.data.games);
        }
    }
}
