//! Time used for a move in a Lichess game. Must have format HH:mm:ss.

pub use super::TIME_FORMAT as FORMAT;

use super::Time;

crate::dtattribute!(Clk, Time);
