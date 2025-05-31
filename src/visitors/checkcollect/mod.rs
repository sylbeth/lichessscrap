//! Module containing all the different PGN reader visitors used for collecting the available data and checking the validity of the data.

use std::io::{self, Write};

use log::trace;
use pgn_reader::{RawComment, RawHeader, Visitor};
use shakmaty::Outcome;

#[cfg(feature = "full-check")]
use pgn_reader::SanPlus;

use super::comment_iterator::CommentIterator;
use checker::Checker;
use collector::Collector;

pub mod checker;
pub mod collector;

/// Visitor that checks the validity of the data and collects the different available fields.
#[derive(Debug, Default)]
pub struct CheckerCollector {
    /// Checker of the validity of the data.
    pub checker: Checker,
    /// Collector of the available fields.
    pub collector: Collector,
}

impl CheckerCollector {
    /// Writes the collection of fields to the writer.
    pub fn write_collection<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        trace!("CheckerCollector write_collection function.");
        self.collector.write_headers(writer)?;
        write!(writer, "\n\n")?;
        self.collector.write_comments(writer)
    }
}

impl Visitor for CheckerCollector {
    type Result = ();

    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        self.collector.collect_header(_key);
        self.checker.check_header(_key, _value.0);
    }

    #[cfg(feature = "full-check")]
    fn san(&mut self, _san: SanPlus) {
        self.checker.san(_san)
    }

    fn comment(&mut self, _comment: RawComment<'_>) {
        for (key, value) in CommentIterator::new(_comment.0) {
            self.collector.collect_comment(key);
            self.checker.check_comment(key, value);
        }
    }

    fn outcome(&mut self, _outcome: Option<Outcome>) {
        self.checker.outcome(_outcome);
    }

    fn end_game(&mut self) {
        self.checker.end_game();
    }
}
