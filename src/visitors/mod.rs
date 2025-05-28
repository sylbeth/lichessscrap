//! Module containing all the different PGN reader visitors that this application uses.

pub mod checkcollect;
pub mod comment_iterator;
pub mod crawler;
#[cfg(any(feature = "data", feature = "relations"))]
pub mod serializer;
#[cfg(feature = "stats")]
pub mod stats;
