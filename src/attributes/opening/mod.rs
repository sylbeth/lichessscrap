//! Module containing all the attributes regarding the opening of a Lichess game.

use mysql::{Params, params};

use eco::Eco;

pub mod eco;

crate::sattribute!(Opening);

impl Opening {
    /// Gets the parameters for MySQL insertion.
    pub fn as_insert_params(&self, eco: Eco) -> Params {
        params! {
            "name" => &self.0,
            "eco_letter" => eco.0,
            "eco_number" => eco.1.get(),
        }
    }

    /// Gets the parameters for MySQL selection.
    pub fn as_select_params(&self) -> Params {
        params! {
            "name" => &self.0
        }
    }
}
