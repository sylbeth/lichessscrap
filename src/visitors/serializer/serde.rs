use std::fs::File;

use csv::Writer;

use super::LichessSerializer;

use super::{DataSerializer, GAMES_CSV, MOVES_CSV};
use lichess::data::{game::Game, r#move::Move};

#[derive(Debug)]
pub struct SerdeSerializer {
    games: Writer<File>,
    moves: Writer<File>,
}

impl LichessSerializer for SerdeSerializer {
    fn new() -> Self {
        Self {
            games: Writer::from_path(GAMES_CSV).expect("The creation of the games csv failed."),
            moves: Writer::from_path(MOVES_CSV).expect("The creation of the moves csv failed."),
        }
    }
}

impl DataSerializer for SerdeSerializer {
    fn write_game(&mut self, game: &Game) {
        self.games
            .serialize(Game { ..game.clone() })
            .expect("The writing of the games csv failed.");
    }

    fn write_move(&mut self, r#move: &Move) {
        self.moves
            .serialize(Move { ..r#move.clone() })
            .expect("The writing of the moves csv failed.");
    }
}
