//! The entire information a Lichess game's move provides. It can be cleared up for reusability purposes.

use mysql::{Params, params};

use crate::attributes::{Clk, Eval, MoveDescriptor};

/// Struct containing all the information of a Lichess game's move.
#[derive(Debug, Default, Clone)]
pub struct Move {
    /// Number of the move, it is only valid if nonzero.
    pub num: usize,
    /// Actual value of the move.
    pub descriptor: MoveDescriptor,
    /// Stockfish evaluation of the move, if any.
    pub eval: Option<Eval>,
    /// Clock time of the move, if any.
    pub clk: Option<Clk>,
}

impl Move {
    /// Resets the move counter.
    pub const fn reset(&mut self) {
        self.num = 0;
    }

    /// Advances the move counter and resets the move's fields.
    pub fn next(&mut self) {
        self.num += 1;
        self.descriptor = MoveDescriptor::default();
        self.eval = None;
        self.clk = None;
    }

    /// Gets the parameters for MySQL insertion.
    pub fn as_params(&self, game_id: u64) -> Params {
        let (eval_float, eval_int) = match self.eval {
            Some(Eval::Numeric(num)) => (Some(num), None),
            Some(Eval::Checkmate(checkmate)) => (None, Some(checkmate)),
            None => (None, None),
        };
        params! {
            game_id,
            "num" => self.num,
            "descriptor" => self.descriptor.to_u32(),
            eval_float,
            eval_int,
            "clock" => self.clk.map(|clk| clk.0.0),
        }
    }
}
