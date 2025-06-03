//! The ruleset (event) of a Lichess game. It can be part of a tournament or a simple game ruleset.

use std::{fmt::Display, str::from_utf8};

use crate::{attribute_err, attribute_fmt};

use super::error::AttributeParsingError;

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
use mysql::{Params, params, prelude::FromValue};

/// A Lichess game's ruleset.
#[derive(Debug, Default, Clone)]
pub struct RuleSet {
    /// The name of the ruleset.
    pub name: String,
    /// The kind of the ruleset.
    pub kind: RuleSetKind,
    /// The url id that this ruleset has.
    pub url: String,
}

impl RuleSet {
    pub fn reset(&mut self) {
        self.name.clear();
        self.kind = RuleSetKind::Game;
        self.url.clear();
    }

    /// Tries to fill the [`RuleSet`]'s data using a `&str`.
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to fill this [`RuleSet`] using a string slice.
    pub fn fill_str(&mut self, value: &str) -> Result<(), AttributeParsingError> {
        let mut split_iter = value.split_terminator(' ').rev();
        if let Some(last_split) = split_iter.next() {
            if last_split == "game" {
                for part in split_iter.rev() {
                    self.name.push_str(part);
                    self.name.push(' ');
                }
                self.name.pop();
                Ok(())
            } else if last_split.starts_with("https") {
                let mut peek_split = split_iter.peekable();
                let kind = match peek_split
                    .next_if(|value| (*value == "tournament") | (*value == "swiss"))
                {
                    Some("tournament") => RuleSetKind::Arena,
                    Some("swiss") => RuleSetKind::Swiss,
                    _ => return Err(ERROR),
                };
                for part in peek_split.rev() {
                    self.name.push_str(part);
                    self.name.push(' ');
                }
                self.name.pop();
                self.kind = kind;
                last_split
                    .split_terminator('/')
                    .rev()
                    .next()
                    .map(|value| self.url.push_str(value));
                Ok(())
            } else {
                Err(ERROR)
            }
        } else {
            Err(ERROR)
        }
    }

    /// Tries to fill the [`RuleSet`]'s data using a `&[u8]`.
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to fill this [`RuleSet`] using a bytes slice.
    pub fn fill_ascii(&mut self, value: &[u8]) -> Result<(), AttributeParsingError> {
        self.fill_str(from_utf8(value).map_err(|_| ERROR)?)
    }

    #[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
    /// Prepares the parameters for MySQL insertion of this data.
    pub fn as_insert_params(&self) -> Params {
        params! {
            "name" => &self.name,
            "url_id" => &self.url,
            "kind" => self.kind,
        }
    }

    #[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
    /// Prepares the parameters for MySQL selection of this data.
    pub fn as_select_params(&self) -> Params {
        params! {
            "name" => &self.name,
            "url_id" => &self.url,
        }
    }
}

/// The kind of RuleSet.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    any(feature = "time-mysql", feature = "chrono-mysql"),
    derive(FromValue)
)]
#[repr(u8)]
pub enum RuleSetKind {
    /// A normal game.
    #[default]
    Game = 1,
    /// An arena tournament.
    Arena,
    /// A swiss tournament.
    Swiss,
}

impl RuleSetKind {
    /// Retrieves the representation of this [`RuleSetKind`] as a `&'static str`.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Game => "Game",
            Self::Arena => "Arena Tournament",
            Self::Swiss => "Swiss Tournament",
        }
    }
}

impl Display for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind.as_str(), self.name)?;
        if self.url != "" {
            write!(f, " {}", self.url)
        } else {
            Ok(())
        }
    }
}

attribute_fmt!(RuleSet, "{str} <game|tournament|swiss>[ {url}]");
attribute_err!(RuleSet);
