use std::{fmt::Display, str::from_utf8};

use super::error::AttributeParsingError;

#[derive(Debug, Default, Clone, Copy)]
pub struct TimeControl(Option<(u16, u8)>);

impl TimeControl {
    /// Finds the time control separator in a time control bytes slice.
    #[cfg(not(feature = "memchr"))]
    fn find_sep(time_control: &[u8]) -> Option<usize> {
        for (i, c) in time_control.iter().enumerate() {
            if *c == b'+' {
                return Some(i);
            }
        }
        None
    }

    /// Finds the time control separator in a time control bytes slice.
    #[cfg(feature = "memchr")]
    fn find_sep(time_control: &[u8]) -> Option<usize> {
        memchr::memchr(b'+', time_control)
    }

    /// Tries to parse a `&str` as a [`TimeControl`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into a [`TimeControl`].
    pub fn from_str(value: &str) -> Result<Self, AttributeParsingError> {
        if value == "-" {
            Ok(Self(None))
        } else if let Some(sep) = Self::find_sep(value.as_bytes()) {
            if let (Ok(total), Ok(increment)) = (value[..sep].parse(), value[sep + 1..].parse()) {
                Ok(Self(Some((total, increment))))
            } else {
                Err(ERROR)
            }
        } else {
            Err(ERROR)
        }
    }

    /// Tries to parse a `&[u8]` as a [`TimeControl`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this bytes slice into a [`TimeControl`].
    pub fn from_ascii(value: &[u8]) -> Result<Self, AttributeParsingError> {
        if value == b"-" {
            Ok(Self(None))
        } else if let Some(sep) = Self::find_sep(value) {
            if let (Ok(total), Ok(increment)) = (
                from_utf8(&value[..sep]).map_err(|_| ERROR)?.parse(),
                from_utf8(&value[sep + 1..]).map_err(|_| ERROR)?.parse(),
            ) {
                Ok(Self(Some((total, increment))))
            } else {
                Err(ERROR)
            }
        } else {
            Err(ERROR)
        }
    }
}

impl Display for TimeControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some((total, increment)) => write!(f, "{total}+{increment}"),
            None => Ok(()),
        }
    }
}

crate::tattribute!(TimeControl, "<-|{u16}+{u8}>");
