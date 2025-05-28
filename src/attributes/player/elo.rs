//! Specification and parsing of an Elo rating from a Lichess player.

use std::{fmt::Display, str::from_utf8};

use super::super::error::AttributeParsingError;

/// Elo rating from a Lichess player.
#[derive(Debug, Default, Clone, Copy)]
pub struct Elo(pub Option<u16>);

impl Elo {
    /// Tries to parse a `&str` as a [`u16`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into an [`Elo`]'s number.
    fn parse_elo_num(value: &str) -> Result<u16, AttributeParsingError> {
        value.parse().map_err(|_| ERROR)
    }

    /// Tries to parse a `&str` as an [`Elo`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into an [`Elo`].
    pub fn from_str(value: &str) -> Result<Self, AttributeParsingError> {
        if value == "?" {
            Ok(Self(None))
        } else {
            Ok(Self(Some(Self::parse_elo_num(value)?)))
        }
    }

    /// Tries to parse a `&[u8]` as an [`Elo`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this bytes slice into an [`Elo`].
    pub fn from_ascii(value: &[u8]) -> Result<Self, AttributeParsingError> {
        if value == b"?" {
            Ok(Self(None))
        } else {
            Ok(Self(Some(Self::parse_elo_num(
                from_utf8(value).map_err(|_| ERROR)?,
            )?)))
        }
    }
}

impl Display for Elo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(value) => value.fmt(f),
            None => Ok(()),
        }
    }
}

crate::tattribute!(Elo, "<?|{u16}>");
