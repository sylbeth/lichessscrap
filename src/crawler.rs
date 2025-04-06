use std::{error::Error, fs::File, io::Write};

use pgn_reader::{Nag, Outcome, RawComment, RawHeader, SanPlus, Visitor};

use crate::{
    collector::Collector, comment_iterator::CommentIterator, game::Game, r#move::Move,
    serializer::Serializer, stats::Stats,
};

pub const GAMES_CSV: &str = "games.csv";
pub const MOVES_CSV: &str = "moves.csv";

#[derive(Debug)]
pub struct Crawler {
    pub game: Game,
    pub r#move: Move,
    pub stats: Stats,
    pub collector: Collector,
    pub serializer: Serializer,
}

impl Crawler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut serializer = Serializer {
            games: File::create(GAMES_CSV)?,
            moves: File::create(MOVES_CSV)?,
        };
        serializer.games.write("Id,Eco,Event,Opening,Result,Site,Termination,TimeControl,UtcDate,UtcTime,White,WhiteElo,WhiteRatingDiff,WhiteTitle,Black,BlackElo,BlackRatingDiff,BlackTitle\n".as_bytes())?;
        serializer.moves.write("GameId,Num,San,Nag,Clk,Eval\n".as_bytes())?;
        Ok(Self {
            game: Game::default(),
            r#move: Move::default(),
            stats: Stats::default(),
            collector: Collector::default(),
            serializer,
        })
    }
}

impl Visitor for Crawler {
    type Result = ();

    fn begin_game(&mut self) {
        self.game.id += 1;
        self.r#move.game_id += 1;
        self.r#move.num = 0;
    }

    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        //self.collector.collect_header(_key);
        self.game.set(_key, _value.0);
        self.stats.headers += 1;
    }

    fn san(&mut self, _san: SanPlus) {
        if self.r#move.num != 0 {
            self.serializer.write_move(&self.r#move).expect("The writing of the moves csv failed.");
            self.r#move.reset();
        }
        self.r#move.num += 1;
        self.r#move.san = _san.to_string();
        self.stats.sans += 1;
    }

    fn nag(&mut self, _nag: Nag) {
        self.r#move.nag = Some(_nag.0);
        self.stats.nags += 1;
    }

    fn comment(&mut self, _comment: RawComment<'_>) {
        let comments = CommentIterator::new(_comment.0);
        for (key, value) in comments {
            //self.collector.collect_comment(key);
            self.r#move.set(key, value);
        }
        self.stats.comments += 1;
    }

    fn end_variation(&mut self) {
        self.stats.variations += 1;
    }

    fn outcome(&mut self, _outcome: Option<Outcome>) {
        self.stats.outcomes += 1;
    }

    fn end_game(&mut self) {
        self.serializer.write_game(&self.game).expect("The writing of the games csv failed.");
        self.serializer.write_move(&self.r#move).expect("The writing of the moves csv failed.");
        self.game.reset();
        self.r#move.reset();
        self.stats.games += 1;
    }
}
