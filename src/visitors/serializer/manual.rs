use std::{
    fs::File,
    io::{BufWriter, Write},
};

use super::LichessSerializer;

use super::{DataSerializer, GAMES_CSV, MOVES_CSV};
use lichess::data::{game::Game, r#move::Move};

#[derive(Debug)]
pub struct ManualSerializer {
    games: BufWriter<File>,
    moves: BufWriter<File>,
}

impl LichessSerializer for ManualSerializer {
    fn new() -> Self {
        let mut manual = Self {
            games: BufWriter::new(
                File::create(GAMES_CSV).expect("The creation of the games csv failed."),
            ),
            moves: BufWriter::new(
                File::create(MOVES_CSV).expect("The creation of the moves csv failed."),
            ),
        };
        manual.games.write(b"GameId,Site,TimeControl,Result,Termination,Date,UTCDate,UTCTime,Opening,ECO,Event,Round,White,WhiteElo,WhiteTitle,WhiteRatingDiff,Black,BlackElo,BlackTitle,BlackRatingDiff").expect("The writing of the header of the games csv failed.");
        manual
            .moves
            .write(b"GameId,Num,San,Nag,Eval,Clk")
            .expect("The writing of the header of the moves csv failed.");
        manual
    }
}

impl DataSerializer for ManualSerializer {
    fn write_game(&mut self, game: &Game) {
        self.games.write(b"\n").unwrap();
        write!(self.games, "{}", game.game_id).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.site.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.time_control.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.result.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.termination.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.date.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.utc_date.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.utc_time.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.opening.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.eco.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.event.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.round.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.white.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.white_elo.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.white_rating_diff.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.white_title.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.black.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.black_elo.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.black_rating_diff.as_bytes()).unwrap();
        self.games.write(b",").unwrap();
        self.games.write(game.black_title.as_bytes()).unwrap();
    }

    fn write_move(&mut self, r#move: &Move) {
        self.moves.write(b"\n").unwrap();
        write!(self.moves, "{}", r#move.game_id).unwrap();
        self.moves.write(b",").unwrap();
        write!(self.moves, "{}", r#move.num).unwrap();
        self.moves.write(b",").unwrap();
        self.moves.write(r#move.san.as_bytes()).unwrap();
        self.moves.write(b",").unwrap();
        if let Some(nag) = &r#move.nag {
            write!(self.moves, "{}", nag).unwrap();
        }
        self.moves.write(b",").unwrap();
        self.moves.write(r#move.eval.as_bytes()).unwrap();
        self.moves.write(b",").unwrap();
        self.moves.write(r#move.clk.as_bytes()).unwrap();
    }
}
