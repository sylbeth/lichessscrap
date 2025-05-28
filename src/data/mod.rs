//! The entire information a Lichess game and its moves provide. They can be cleared up for reusability purposes.

pub use game::Game;
pub use r#move::Move;
use pgn_reader::{Nag, SanPlus};
use shakmaty::Position;

use super::attributes::{error::ValuedAttributeParsingError, r#move as inner_move};

pub mod game;
pub mod r#move;

/// All the data in a PGN file as it is being read.
#[derive(Debug, Default)]
pub struct Data {
    /// The number of games read.
    pub games: usize,
    /// The current game being analyzed.
    pub game: Game,
    /// The current move being analyzed.
    pub r#move: Move,
}

impl Data {
    /// When a new game happens, game and move must be reset.
    pub fn new_game(&mut self) {
        self.game.reset();
        self.r#move.reset();
    }

    /// When a new move happens, move is advanced and parsed to board.
    pub fn new_move(&mut self, san: SanPlus) -> Result<(), ValuedAttributeParsingError> {
        self.r#move.next();
        let current_move = san.san.to_move(&self.game.chess).map_err(|_| {
            ValuedAttributeParsingError::from_inner_utf8(
                inner_move::ERROR,
                format!("{san} at move {} of game {}", self.r#move.num, self.games),
            )
        })?;
        let color = self.game.chess.turn();
        self.game.chess.play_unchecked(&current_move);
        inner_move::Move::from_move(current_move, san.suffix, color).map_err(|_| {
            ValuedAttributeParsingError::from_inner_utf8(
                inner_move::ERROR,
                format!("{san} at move {} of game {}", self.r#move.num, self.games),
            )
        })?;
        Ok(())
    }

    /// Adds the nag to the move's current value.
    pub fn add_nag(&mut self, nag: Nag) {
        self.r#move.r#move.nag = nag;
    }

    /// Checks whether the current move is valid or not.
    pub fn is_move_valid(&self) -> bool {
        self.r#move.num != 0
    }
}
