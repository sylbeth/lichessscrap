pub mod constants;
#[cfg(feature = "data")]
pub mod data;

pub mod prelude {
    pub use super::constants::{comments::*, headers::*};
    #[cfg(feature = "data")]
    pub use super::data::{Data, game::Game, r#move::Move};
}
