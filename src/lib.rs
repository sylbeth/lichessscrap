//! Library module of the scrapper, which contains as submodules all the logic for the data.

pub mod attributes;
pub mod constants;
pub mod data;

pub mod prelude {
    pub use super::attributes::*;
    pub use super::constants::{comments::*, headers::*};
    pub use super::data::{Data, game::Game, r#move::Move};
}
