//! The ruleset of a Lichess game. It can be part of a tournament or a simple game ruleset.

use std::{fmt::Display, str::from_utf8};

use mysql::prelude::FromValue;

use crate::{attribute_err, attribute_fmt};

use super::error::AttributeParsingError;

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

    /// Tries to parse a `&str` as an [`RuleSet`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into an [`Elo`].
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

    /// Tries to parse a `&[u8]` as an [`RuleSet`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this bytes slice into an [`RuleSet`].
    pub fn fill_ascii(&mut self, value: &[u8]) -> Result<(), AttributeParsingError> {
        self.fill_str(from_utf8(value).map_err(|_| ERROR)?)
    }
}

/// The kind of RuleSet.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromValue)]
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
