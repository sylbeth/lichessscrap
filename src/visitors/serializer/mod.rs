#[cfg(feature = "data")]
use lichess::data::{game::Game, r#move::Move};

#[cfg(all(feature = "csv", not(feature = "serde")))]
mod csv;
#[cfg(not(feature = "csv"))]
mod manual;
#[cfg(feature = "serde")]
mod serde;

#[cfg(all(feature = "csv", not(feature = "serde"), not (feature = "mysql")))]
pub use csv::CSVSerializer as Serializer;
#[cfg(all(not(feature = "csv"), not(feature= "mysql")))]
pub use manual::ManualSerializer as Serializer;
#[cfg(all(feature = "serde", not(feature = "mysql")))]
pub use serde::SerdeSerializer as Serializer;

#[cfg(feature = "mysql")]
pub mod db;

#[cfg(feature = "mysql")]
pub use db::DbSerializer as Serializer;
#[cfg(feature = "mysql")]
pub use mysql::PooledConn;



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


pub trait MysqlSerializer {
    fn new(database_url: &str) -> Self;
    fn get_or_create_player(&self, name: &str) -> u64;
    fn get_or_create_opening(&self, name: &str, eco_code: &str) -> u64;
}
    