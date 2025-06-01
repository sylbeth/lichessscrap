//! Module containing all the attributes regarding a player of Lichess.

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
use mysql::{Params, params};

pub mod elo;
pub mod title;

crate::sattribute!(Player);

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
impl Player {
    /// Prepares the parameters for MySQL insertion and selection of this data.
    pub fn as_params(&self) -> Params {
        params! {
            "name" => &self.0,
        }
    }
}
