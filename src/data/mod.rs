pub use game::Game;
pub use r#move::Move;
use pgn_reader::{Nag, SanPlus};

pub mod game;
pub mod r#move;

#[derive(Debug, Default)]
pub struct Data {
    pub game: Game,
    pub r#move: Move,
}

impl Data {
    pub fn new_game(&mut self) {
        self.game.game_id += 1;
        self.game.reset();
        self.r#move.game_id += 1;
        self.r#move.num = 0;
        self.r#move.reset();
    }

    pub fn new_move(&mut self, san: SanPlus) {
        self.r#move.reset();
        self.r#move.num += 1;
        self.r#move.san = san.to_string();
    }

    pub fn add_nag(&mut self, nag: Nag) {
        self.r#move.nag = Some(nag.0);
    }

    pub fn is_move_valid(&self) -> bool {
        self.r#move.num != 0
    }
}
