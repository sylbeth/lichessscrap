//! UTC Time of the day in which a Lichess game took place.  Must have format HH:mm:ss.

pub use super::TIME_FORMAT as FORMAT;

use super::Time;

crate::dtattribute!(UTCTime, Time);
