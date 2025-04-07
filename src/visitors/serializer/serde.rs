use std::fs::File;

use csv::Writer;

use super::LichessSerializer;

#[cfg(feature = "data")]
use super::{DataSerializer, GAMES_CSV, MOVES_CSV};
#[cfg(feature = "data")]
use lichess::data::{game::Game, r#move::Move};

#[derive(Debug)]
pub struct SerdeSerializer {
    #[cfg(feature = "data")]
    games: Writer<File>,
    #[cfg(feature = "data")]
    moves: Writer<File>,
}

impl LichessSerializer for SerdeSerializer {
    fn new() -> Self {
        Self {
            #[cfg(feature = "data")]
            games: Writer::from_path(GAMES_CSV).expect("The creation of the games csv failed."),
            #[cfg(feature = "data")]
            moves: Writer::from_path(MOVES_CSV).expect("The creation of the moves csv failed."),
        }
    }
}

#[cfg(feature = "data")]
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
