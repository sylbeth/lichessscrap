pub mod constants;
pub mod data;

pub mod prelude {
    pub use super::constants::{comments::*, headers::*};
    pub use super::data::{Data, game::Game, r#move::Move};
}
