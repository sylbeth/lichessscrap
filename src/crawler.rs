use std::error::Error;

use csv::Writer;
use pgn_reader::{Nag, Outcome, RawComment, RawHeader, SanPlus, Visitor};

use crate::{collector::Collector, serializer::Serializer, stats::Stats};

pub const GAMES_CSV: &str = "games.csv";
pub const MOVES_CSV: &str = "moves.csv";

#[derive(Debug)]
pub struct Crawler {
    pub stats: Stats,
    pub collector: Collector,
    pub serializer: Serializer,
}

impl Crawler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            stats: Stats::default(),
            collector: Collector::default(),
            serializer: Serializer {
                games: Writer::from_path(GAMES_CSV)?,
                moves: Writer::from_path(MOVES_CSV)?,
            },
        })
    }
}

impl Visitor for Crawler {
    type Result = ();

    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        self.collector.collect_header(_key);
        self.stats.headers += 1;
    }

    fn san(&mut self, _san: SanPlus) {
        self.stats.sans += 1;
    }

    fn nag(&mut self, _nag: Nag) {
        self.stats.nags += 1;
    }

    fn comment(&mut self, _comment: RawComment<'_>) {
        self.collector.collect_comment(_comment.0);
        self.stats.comments += 1;
    }

    fn end_variation(&mut self) {
        self.stats.variations += 1;
    }

    fn outcome(&mut self, _outcome: Option<Outcome>) {
        self.stats.outcomes += 1;
    }

    fn end_game(&mut self) {
        self.stats.games += 1;
    }
}
