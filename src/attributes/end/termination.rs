//! Termination of a Lichess game. Can be any way a game can end.

use deranged::RangedU8;
use mysql::prelude::FromValue;

use super::super::error::AttributeParsingError;

/// All possible [`Termination`]s, ensuring the format is exhaustive.
const ALL_TERMINATIONS: [&str; 5] = [
    NORMAL_STR,
    TIME_FORFEIT_STR,
    RULES_INFRACTION_STR,
    ABANDONED_STR,
    UNTERMINATED_STR,
];

/// ASCII string slice representing a termination by checkmate.
const NORMAL: &[u8] = NORMAL_STR.as_bytes();
/// ASCII string slice representing a termination by time.
const TIME_FORFEIT: &[u8] = TIME_FORFEIT_STR.as_bytes();
/// ASCII string slice representing a termination by an infraction of rules.
const RULES_INFRACTION: &[u8] = RULES_INFRACTION_STR.as_bytes();
/// ASCII string slice representing a termination by an abandonment.
const ABANDONED: &[u8] = ABANDONED_STR.as_bytes();
/// ASCII string slice representing a null termination.
const UNTERMINATED: &[u8] = UNTERMINATED_STR.as_bytes();

/// UTF-8 string slice representing a termination by checkmate.
const NORMAL_STR: &str = "Normal";
/// UTF-8 string slice representing a termination by time.
const TIME_FORFEIT_STR: &str = "Time forfeit";
/// UTF-8 string slice representing a termination by an infraction of rules.
const RULES_INFRACTION_STR: &str = "Rules infraction";
/// UTF-8 string slice representing a termination by an abandonment.
const ABANDONED_STR: &str = "Abandoned";
/// UTF-8 string slice representing a null termination.
const UNTERMINATED_STR: &str = "Unterminated";

/// Termination of a Lichess game.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromValue)]
#[repr(u8)]
pub enum Termination {
    /// A null termination.
    #[default]
    Unterminated = 1,
    /// A termination by checkmate.
    Normal,
    /// A termination by time.
    TimeForfeit,
    /// A termination by an infraction of rules.
    RulesInfraction,
    /// A termination by an abandonment.
    Abandoned,
}

impl Termination {
    /// Retrieves the representation of this [`Termination`] as a [`u8`], a value between 0 and 5.
    pub const fn as_u8(&self) -> u8 {
        (*self as u8) - 1
    }

    /// Retrieves the representation of this [`Termination`] as a [`RangedU8`], a value between 0 and 5.
    pub const fn as_ranged(&self) -> RangedU8<0, 4> {
        RangedU8::new((*self as u8) - 1).expect("There are only 5 enum variants, this must work.")
    }

    /// Retrieves the representation of this [`Termination`] as a `&'static str`.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => NORMAL_STR,
            Self::TimeForfeit => TIME_FORFEIT_STR,
            Self::RulesInfraction => RULES_INFRACTION_STR,
            Self::Abandoned => ABANDONED_STR,
            Self::Unterminated => UNTERMINATED_STR,
        }
    }

    /// Retrieves the representation of this [`Termination`] as a `&'static [u8]`.
    pub const fn as_ascii(&self) -> &'static [u8] {
        match self {
            Self::Normal => NORMAL,
            Self::TimeForfeit => TIME_FORFEIT,
            Self::RulesInfraction => RULES_INFRACTION,
            Self::Abandoned => ABANDONED,
            Self::Unterminated => UNTERMINATED,
        }
    }

    /// Tries to parse a `&str` as a [`Termination`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into a [`Termination`].
    pub const fn from_str(value: &str) -> Result<Self, AttributeParsingError> {
        Self::from_ascii(value.as_bytes())
    }

    /// Tries to parse a `&[u8]` as a [`Termination`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this bytes slice into a [`Termination`].
    pub const fn from_ascii(value: &[u8]) -> Result<Self, AttributeParsingError> {
        match value {
            NORMAL => Ok(Self::Normal),
            TIME_FORFEIT => Ok(Self::TimeForfeit),
            RULES_INFRACTION => Ok(Self::RulesInfraction),
            ABANDONED => Ok(Self::Abandoned),
            UNTERMINATED => Ok(Self::Unterminated),
            _ => Err(ERROR),
        }
    }
}

crate::eattribute!(Termination, &ALL_TERMINATIONS);
