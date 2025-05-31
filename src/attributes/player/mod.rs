//! Module containing all the attributes regarding a player of Lichess.

use mysql::{Params, params};

pub mod elo;
pub mod title;

crate::sattribute!(Player);

impl Player {
    /// Gets the parameters for MySQL insertion and selection.
    pub fn as_params(&self) -> Params {
        params! {
            "name" => &self.0,
        }
    }
}
