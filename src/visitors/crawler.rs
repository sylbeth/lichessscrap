use pgn_reader::{Nag, Outcome, RawComment, RawHeader, SanPlus, Visitor};

use super::{
    comment_iterator::CommentIterator,
    serializer::{LichessSerializer, Serializer},
};

#[cfg(feature = "collection")]
use super::collector::Collector;

#[cfg(feature = "data")]
use super::serializer::DataSerializer;
#[cfg(feature = "data")]
use lichess::data::Data;

#[cfg(feature = "stats")]
use super::stats::Stats;
#[cfg(feature = "stats")]
use pgn_reader::San;
#[cfg(feature = "stats")]
const NULL_SAN: SanPlus = SanPlus {
    san: San::Null,
    suffix: None,
};
#[cfg(feature = "stats")]
const NULL_NAG: Nag = Nag { 0: 0 };

#[derive(Debug)]
pub struct Crawler {
    #[cfg(feature = "data")]
    pub data: Data,
    #[cfg(feature = "stats")]
    pub stats: Stats,
    #[cfg(feature = "collection")]
    pub collector: Collector,
    pub serializer: Serializer,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "data")]
            data: Data::default(),
            #[cfg(feature = "stats")]
            stats: Stats::default(),
            #[cfg(feature = "collection")]
            collector: Collector::default(),
            serializer: Serializer::new(),
        }
    }

    pub fn show(&self) {
        #[cfg(feature = "collection")]
        {
            self.collector.print_headers();
            println!();
            self.collector.print_comments();
            println!();
        }
        #[cfg(feature = "stats")]
        print!("{}", self.stats);
    }
}

impl Visitor for Crawler {
    type Result = ();

    fn begin_game(&mut self) {
        #[cfg(all(feature = "stats", feature = "log"))]
        if self.stats.games % 100000 == 0 {
            println!("Processed {} games.", self.stats.games);
        }
        #[cfg(feature = "data")]
        self.data.new_game();
    }

    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        #[cfg(feature = "collection")]
        self.collector.collect_header(_key);
        #[cfg(feature = "data")]
        self.data.game.set(_key, _value.0);
        #[cfg(feature = "stats")]
        self.stats.header(_key, _value);
    }

    fn san(&mut self, _san: SanPlus) {
        #[cfg(feature = "data")]
        if self.data.is_move_valid() {
            self.serializer.write_move(&self.data.r#move);
        }
        #[cfg(feature = "data")]
        self.data.new_move(_san);
        #[cfg(feature = "stats")]
        self.stats.san(NULL_SAN);
    }

    fn nag(&mut self, _nag: Nag) {
        #[cfg(feature = "data")]
        self.data.add_nag(_nag);
        #[cfg(feature = "stats")]
        self.stats.nag(NULL_NAG);
    }

    fn comment(&mut self, _comment: RawComment<'_>) {
        let comments = CommentIterator::new(_comment.0);
        for (key, value) in comments {
            #[cfg(feature = "collection")]
            self.collector.collect_comment(key);
            #[cfg(feature = "data")]
            self.data.r#move.set(key, value);
        }
        #[cfg(feature = "stats")]
        self.stats.comment(RawComment(&[]));
    }

    fn end_variation(&mut self) {
        #[cfg(feature = "stats")]
        self.stats.end_variation();
    }

    fn outcome(&mut self, _outcome: Option<Outcome>) {
        #[cfg(feature = "stats")]
        self.stats.outcome(None);
    }

    fn end_game(&mut self) {
        #[cfg(feature = "data")]
        {
            self.serializer.write_game(&self.data.game);
            self.serializer.write_move(&self.data.r#move);
        }
        #[cfg(feature = "stats")]
        self.stats.end_game();
    }
}
