//! Constants used for matching the headers to the kind of attribute their values are.

/// Website header of a game.
pub const SITE: &[u8] = b"Site";

/// Lichess ID.
pub const LICHESS_ID: &[u8] = b"LichessId";

/// Time control header of a game.
pub const TIME_CONTROL: &[u8] = b"TimeControl";
/// Result header of a game.
pub const RESULT: &[u8] = b"Result";
/// Termination header of a game.
pub const TERMINATION: &[u8] = b"Termination";

/// Date header of a game.
pub const DATE: &[u8] = b"Date";
/// UTC date header of a game.
pub const UTC_DATE: &[u8] = b"UTCDate";
/// UTC time header of a game.
pub const UTC_TIME: &[u8] = b"UTCTime";

/// Event header of a game.
pub const EVENT: &[u8] = b"Event";
/// Round header of a game.
pub const ROUND: &[u8] = b"Round";

/// Opening header of a game.
pub const OPENING: &[u8] = b"Opening";
/// ECO code header of a game.
pub const ECO: &[u8] = b"ECO";

/// Black player header of a game.
pub const BLACK: &[u8] = b"Black";
/// Black player's elo header of a game.
pub const BLACK_ELO: &[u8] = b"BlackElo";
/// Black player's rating difference header of a game.
pub const BLACK_RATING_DIFF: &[u8] = b"BlackRatingDiff";
/// Black player's title header of a game.
pub const BLACK_TITLE: &[u8] = b"BlackTitle";

/// White player header of a game.
pub const WHITE: &[u8] = b"White";
/// White player's elo header of a game.
pub const WHITE_ELO: &[u8] = b"WhiteElo";
/// White player's rating difference header of a game.
pub const WHITE_RATING_DIFF: &[u8] = b"WhiteRatingDiff";
/// White player's title header of a game.
pub const WHITE_TITLE: &[u8] = b"WhiteTitle";
