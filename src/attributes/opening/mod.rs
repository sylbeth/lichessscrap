//! Module containing all the attributes regarding the opening of a Lichess game.

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
use eco::Eco;
#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
use mysql::{Params, params};

pub mod eco;

crate::sattribute!(Opening);

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
impl Opening {
    /// Prepares the parameters for MySQL insertion of this data.
    pub fn as_insert_params(&self, eco: Eco) -> Params {
        params! {
            "name" => &self.0,
            "eco_letter" => eco.0,
            "eco_number" => eco.1.get(),
        }
    }

    /// Prepares the parameters for MySQL selection of this data.
    pub fn as_select_params(&self) -> Params {
        params! {
            "name" => &self.0
        }
    }
}
