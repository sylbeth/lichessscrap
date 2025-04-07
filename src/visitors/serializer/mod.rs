#[cfg(feature = "data")]
use lichess::data::{game::Game, r#move::Move};

#[cfg(all(feature = "csv", not(feature = "serde")))]
mod csv;
#[cfg(not(feature = "csv"))]
mod manual;
#[cfg(feature = "serde")]
mod serde;

#[cfg(all(feature = "csv", not(feature = "serde")))]
pub use csv::CSVSerializer as Serializer;
#[cfg(not(feature = "csv"))]
pub use manual::ManualSerializer as Serializer;
#[cfg(feature = "serde")]
pub use serde::SerdeSerializer as Serializer;

pub const GAMES_CSV: &str = "games.csv";
pub const MOVES_CSV: &str = "moves.csv";

pub trait LichessSerializer {
    fn new() -> Self;
}

#[cfg(feature = "data")]
pub trait DataSerializer {
    fn write_game(&mut self, game: &Game);
    fn write_move(&mut self, r#move: &Move);
}
