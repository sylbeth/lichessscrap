//! Module with all the headers, comments and evaluated attributes from the reading of the PGN file. It handles parsing and data conversion.

use std::fmt::Display;

pub mod attribute;
pub mod datetime;
pub mod end;
pub mod error;
pub mod eval;
pub mod r#move;
pub mod opening;
pub mod pieces;
pub mod player;
pub mod ruleset;
pub mod time_control;

pub use self::{
    end::{result::Result, termination::Termination},
    eval::Eval,
    opening::{
        Opening,
        eco::{Eco, EcoChar},
    },
    player::{Player, elo::Elo, title::Title},
    ruleset::RuleSet,
    time_control::TimeControl,
};

/// The kind of attribute used for parsing correctness.
#[derive(Debug, Clone, Copy)]
pub enum AttributeKind {
    Result,
    Termination,
    TimeControl,
    Date,
    UTCDate,
    UTCTime,
    RuleSet,
    Opening,
    Eco,
    Player,
    Elo,
    Title,
    Eval,
    Clk,
    Move,
    PiecesLeft,
}

impl AttributeKind {
    /// Getter of the format of the kind of attribute.
    const fn format(&self) -> AttributeFormat {
        match self {
            Self::Result => end::result::FORMAT,
            Self::Termination => end::termination::FORMAT,
            Self::TimeControl => time_control::FORMAT,
            Self::Date => datetime::date::FORMAT,
            Self::UTCDate => datetime::utc_date::FORMAT,
            Self::UTCTime => datetime::utc_time::FORMAT,
            Self::RuleSet => ruleset::FORMAT,
            Self::Opening => opening::FORMAT,
            Self::Eco => opening::eco::FORMAT,
            Self::Player => player::FORMAT,
            Self::Elo => player::elo::FORMAT,
            Self::Title => player::title::FORMAT,
            Self::Eval => eval::FORMAT,
            Self::Clk => datetime::clk::FORMAT,
            Self::Move => r#move::FORMAT,
            Self::PiecesLeft => pieces::FORMAT,
        }
    }
}

impl Display for AttributeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Result => "result",
            Self::Termination => "termination",
            Self::TimeControl => "time control",
            Self::Date => "date",
            Self::UTCDate => "utc date",
            Self::UTCTime => "utc time",
            Self::RuleSet => "ruleset",
            Self::Opening => "opening",
            Self::Eco => "eco",
            Self::Player => "player",
            Self::Elo => "elo",
            Self::Title => "title",
            Self::Eval => "%eval",
            Self::Clk => "%clk",
            Self::Move => "move",
            Self::PiecesLeft => "pieces left",
        }
        .fmt(f)
    }
}

/// Format for an attribute kind.
#[derive(Debug)]
pub enum AttributeFormat {
    /// Enumeration format, a list of available values.
    Enum(&'static [&'static str]),
    /// Formatting string, that defines the format.
    Str(&'static str),
}

impl Display for AttributeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enum(kinds) => {
                write!(f, "in [")?;
                for kind in *kinds {
                    write!(f, "{}", kind)?;
                }
                write!(f, "]")
            }
            Self::Str(format) => format.fmt(f),
        }
    }
}
