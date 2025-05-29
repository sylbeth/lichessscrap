//! Result of a Lichess game. Can be a win for each of the sides, a tie or none.

use deranged::RangedU8;
use shakmaty::{Color, Outcome};

use super::super::error::AttributeParsingError;

/// All possible [`Result`]s, ensuring the format is exhaustive.
const ALL_RESULTS: [&str; 4] = [WHITE_WIN_STR, BLACK_WIN_STR, TIE_STR, NULL_STR];

/// ASCII string slice representing a win from the white player.
const WHITE_WIN: &[u8] = WHITE_WIN_STR.as_bytes();
/// ASCII string slice representing a win from the black player.
const BLACK_WIN: &[u8] = BLACK_WIN_STR.as_bytes();
/// ASCII string slice representing a tie between white and black players.
const TIE: &[u8] = TIE_STR.as_bytes();
/// ASCII string slice representing a null result.
const NULL: &[u8] = NULL_STR.as_bytes();

/// UTF-8 string slice representing a win from the white player.
const WHITE_WIN_STR: &str = "1-0";
/// UTF-8 string slice representing a win from the black player.
const BLACK_WIN_STR: &str = "0-1";
/// UTF-8 string slice representing a tie between white and black players.
const TIE_STR: &str = "1/2-1/2";
/// UTF-8 string slice representing a null result.
const NULL_STR: &str = "*";

/// Result of a Lichess game.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Result {
    /// A null result.
    #[default]
    Null,
    /// A win from the white player.
    White,
    /// A win from the black player.
    Black,
    /// A tie between white and black players.
    Tie,
}

impl Result {
    /// Retrieves the representation of this [`Result`] as a [`u8`], a value between 0 and 3.
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Retrieves the representation of this [`Result`] as a [`RangedU8`], a value between 0 and 3.
    pub const fn as_ranged(&self) -> RangedU8<0, 3> {
        RangedU8::new(*self as u8).expect("There are only 4 enum variants, this must work.")
    }

    /// Retrieves the representation of this [`Result`] as a `&'static str`.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::White => WHITE_WIN_STR,
            Self::Black => BLACK_WIN_STR,
            Self::Tie => TIE_STR,
            Self::Null => NULL_STR,
        }
    }

    /// Retrieves the representation of this [`Result`] as a `&'static [u8]`.
    pub const fn as_ascii(&self) -> &'static [u8] {
        match self {
            Self::White => WHITE_WIN,
            Self::Black => BLACK_WIN,
            Self::Tie => TIE,
            Self::Null => NULL,
        }
    }

    /// Tries to parse a `&str` as a [`Result`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into a [`Result`].
    pub const fn from_str(value: &str) -> std::result::Result<Self, AttributeParsingError> {
        Self::from_ascii(value.as_bytes())
    }

    /// Tries to parse a `&[u8]` as a [`Result`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this bytes slice into a [`Result`].
    pub const fn from_ascii(value: &[u8]) -> std::result::Result<Self, AttributeParsingError> {
        match value {
            WHITE_WIN => Ok(Self::White),
            BLACK_WIN => Ok(Self::Black),
            TIE => Ok(Self::Tie),
            NULL => Ok(Self::Null),
            _ => Err(ERROR),
        }
    }
}

impl From<Outcome> for Result {
    fn from(value: Outcome) -> Self {
        match value {
            Outcome::Decisive {
                winner: Color::White,
            } => Self::White,
            Outcome::Decisive {
                winner: Color::Black,
            } => Self::Black,
            Outcome::Draw => Self::Tie,
        }
    }
}

impl From<Option<Outcome>> for Result {
    fn from(value: Option<Outcome>) -> Self {
        match value {
            Some(Outcome::Decisive {
                winner: Color::White,
            }) => Self::White,
            Some(Outcome::Decisive {
                winner: Color::Black,
            }) => Self::Black,
            Some(Outcome::Draw) => Self::Tie,
            None => Self::Null,
        }
    }
}

crate::eattribute!(Result, &ALL_RESULTS);
