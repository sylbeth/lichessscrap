use std::fs::File;

use csv::Writer;

use crate::{game::Game, r#move::Move};

#[derive(Debug)]
pub struct Serializer {
    pub games: Writer<File>,
    pub moves: Writer<File>,
}

impl Serializer {
    pub fn write_game(&mut self, game: &Game) {
        self.games
            .serialize(Game { ..game.clone() })
            .expect("The writing of the games csv failed.");
    }

    pub fn write_move(&mut self, r#move: &Move) {
        self.moves
            .serialize(Move { ..r#move.clone() })
            .expect("The writing of the moves csv failed.");
    }
}
