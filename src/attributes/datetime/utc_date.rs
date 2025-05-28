//! UTC Date in which a Lichess game took place. Must have format YYYY.MM.DD.

pub use super::DATE_FORMAT as FORMAT;

use super::Date;

crate::dtattribute!(UTCDate, Date);
