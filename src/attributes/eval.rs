//! Stockfish evaluation of a move from a Lichess game. Can either be moves left until checkmate or a floating point evaluation.

use std::{
    fmt::Display,
    str::{FromStr, from_utf8},
};

use super::error::AttributeParsingError;

/// A move's Stockfish evaluation.
#[derive(Debug, Clone, Copy)]
pub enum Eval {
    Numeric(f32),
    Checkmate(i8),
}

impl Eval {
    /// Tries to parse a `&str` as a [`f32`]
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into a numeric [`Eval`].
    fn parse_eval<T: FromStr>(value: &str) -> Result<T, AttributeParsingError> {
        value.parse::<T>().map_err(|_| ERROR)
    }

    /// Tries to parse a `&str` as an [`Eval`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into an [`Eval`].
    pub fn from_str(value: &str) -> Result<Self, AttributeParsingError> {
        let mut chars = value.chars();
        if let Some(char) = chars.next() {
            if char == '#' {
                Ok(Self::Checkmate(Self::parse_eval(chars.as_str())?))
            } else {
                Ok(Self::Numeric(Self::parse_eval(value)?))
            }
        } else {
            Err(ERROR)
        }
    }

    /// Tries to parse a `&[u8]` as an [`Eval`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this bytes slice into an [`Eval`].
    pub fn from_ascii(value: &[u8]) -> Result<Self, AttributeParsingError> {
        let mut iter = value.into_iter();
        if let Some(char) = iter.next() {
            if *char == b'#' {
                Ok(Self::Checkmate(Self::parse_eval(
                    from_utf8(iter.as_slice()).map_err(|_| ERROR)?,
                )?))
            } else {
                Ok(Self::Numeric(Self::parse_eval(
                    from_utf8(value).map_err(|_| ERROR)?,
                )?))
            }
        } else {
            Err(ERROR)
        }
    }
}

impl Display for Eval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Numeric(num) => write!(f, "{:.2}", num),
            Self::Checkmate(num) => write!(f, "#{num}"),
        }
    }
}

crate::tattribute!(Eval, "<{f32}|#{i8}>");
