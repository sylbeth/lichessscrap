use pgn_reader::{Nag, Outcome, RawComment, RawHeader, SanPlus, Visitor};

use super::{
    comment_iterator::CommentIterator,
    serializer::{LichessSerializer, Serializer},
};

#[cfg(feature = "collection")]
use super::collector::Collector;

use lichess::data::Data;

use super::stats::Stats;
use pgn_reader::San;
const NULL_SAN: SanPlus = SanPlus {
    san: San::Null,
    suffix: None,
};
const NULL_NAG: Nag = Nag { 0: 0 };

#[derive(Debug)]
pub struct Crawler {
    pub data: Data,
    pub stats: Stats,
    #[cfg(feature = "collection")]
    pub collector: Collector,
    pub serializer: Serializer,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            data: Data::default(),
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
        print!("{}", self.stats);
    }
}

impl Visitor for Crawler {
    type Result = ();

    fn begin_game(&mut self) {
        if self.stats.games % 100000 == 0 {
            println!("Processed {} games.", self.stats.games);
        }
        self.data.new_game();
    }

    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        #[cfg(feature = "collection")]
        self.collector.collect_header(_key);
        self.data.game.set(_key, _value.0);
        self.stats.header(_key, _value);
    }

    fn san(&mut self, _san: SanPlus) {
        if self.data.is_move_valid() {
            self.serializer.write_move(&self.data.r#move);
        }
        self.data.new_move(_san);
        self.stats.san(NULL_SAN);
    }

    fn nag(&mut self, _nag: Nag) {
        self.data.add_nag(_nag);
        self.stats.nag(NULL_NAG);
    }

    fn comment(&mut self, _comment: RawComment<'_>) {
        let comments = CommentIterator::new(_comment.0);
        for (key, value) in comments {
            #[cfg(feature = "collection")]
            self.collector.collect_comment(key);
            self.data.r#move.set(key, value);
        }
        self.stats.comment(RawComment(&[]));
    }

    fn end_variation(&mut self) {
        self.stats.end_variation();
    }

    fn outcome(&mut self, _outcome: Option<Outcome>) {
        self.stats.outcome(None);
    }

    fn end_game(&mut self) {
        {
            self.serializer.write_game(&self.data.game);
            self.serializer.write_move(&self.data.r#move);
        }
        self.stats.end_game();
    }
}
