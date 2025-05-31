//! Module with the error types used when parsing attributes erroneously.

use std::{error::Error, fmt::Display};

use super::AttributeKind;

/// Error for the parsing of the Lichess data attributes.
#[derive(Debug, Clone, Copy)]
pub struct AttributeParsingError(AttributeKind);

impl AttributeParsingError {
    /// Creates a new error by the [`AttributeKind`] that was tried to parse.
    pub const fn new(parsed_kind: AttributeKind) -> Self {
        Self(parsed_kind)
    }
}

impl Display for AttributeParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unable to parse `{}` as it's not formatted correctly ({})",
            self.0,
            self.0.format()
        )
    }
}

impl Error for AttributeParsingError {}

#[derive(Debug, Clone)]
/// Error for the parsing of the Lichess data attributes with the value that was parsed wrongly.
pub struct ValuedAttributeParsingError {
    /// The actual error that has occurred.
    inner: AttributeParsingError,
    /// The bytes value the error has occurred with.
    value: Vec<u8>,
}

impl ValuedAttributeParsingError {
    /// Creates a new error by the [`AttributeKind`] that was tried to parse and the value that was being tried to parse.
    pub const fn new(parsed_kind: AttributeKind, value: Vec<u8>) -> Self {
        Self {
            inner: AttributeParsingError::new(parsed_kind),
            value,
        }
    }

    /// Creates a new error by the [`AttributeKind`] that was tried to parse and the value that was being tried to parse as a [`String`].
    pub fn new_utf8(parsed_kind: AttributeKind, value: String) -> Self {
        Self {
            inner: AttributeParsingError::new(parsed_kind),
            value: value.into_bytes(),
        }
    }

    /// Creates a new error by the [`AttributeParsingError`] that was generated and the value that was being tried to parse.
    pub const fn from_inner(inner: AttributeParsingError, value: Vec<u8>) -> Self {
        Self { inner, value }
    }

    /// Creates a new error by the [`AttributeParsingError`] that was generated and the value that was being tried to parse as a [`String`].
    pub fn from_inner_utf8(inner: AttributeParsingError, value: String) -> Self {
        Self {
            inner,
            value: value.into_bytes(),
        }
    }
}

impl Display for ValuedAttributeParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} <- {:?}",
            self.inner,
            String::from_utf8_lossy(&self.value),
            self.value
        )
    }
}

impl Error for ValuedAttributeParsingError {}

/// Logs an error without more context than the game it happened at.
#[macro_export]
macro_rules! loneerror {
    ($str:literal, $self:ident) => {
        log::error!(concat!("{} - ", $str), $self.games);
        $self.has_errors = true;
    };
}

/// Logs an error about a header of a game that is null.
#[macro_export]
macro_rules! nullerror {
    ($str:literal, $self:ident) => {
        log::error!(concat!("{} - ", $str, " is null"), $self.games);
        $self.has_errors = true;
    };
}

/// Logs an error with context and the game it happened at.
#[macro_export]
macro_rules! valuederror {
    ($self:ident, $error:ident) => {
        log::error!("{} - {}", $self.games, $error);
        $self.has_errors = true;
    };
}
