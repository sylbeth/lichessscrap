//! ECO code of an opening. Starts with a letter from A to E and ends with a number from 0 to 99.

use std::{fmt::Display, str::from_utf8};

use deranged::RangedU8;

use super::super::error::AttributeParsingError;

/// Character at the start of an Eco code.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EcoChar {
    #[default]
    Q,
    A,
    B,
    C,
    D,
    E,
}

impl EcoChar {
    /// Retrieves the representation of this [`EcoChar`] as a [`u8`], a value between 0 and 5.
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Retrieves the representation of this [`EcoChar`] as a [`RangedU8`], a value between 0 and 5.
    pub const fn as_ranged(&self) -> RangedU8<0, 5> {
        RangedU8::new(*self as u8).expect("There are only 5 enum variants, this must work.")
    }

    /// Retrieves the representation of this [`EcoChar`] as a `char`.
    pub const fn as_char(&self) -> char {
        match self {
            Self::Q => '?',
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
        }
    }

    /// Retrieves the representation of this [`EcoChar`] as a `u8`.
    pub const fn as_ascii(&self) -> u8 {
        match self {
            Self::Q => b'?',
            Self::A => b'A',
            Self::B => b'B',
            Self::C => b'C',
            Self::D => b'D',
            Self::E => b'E',
        }
    }

    /// Tries to parse a `char` as an [`EcoChar`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this char into an [`EcoChar`].
    pub const fn from_char(value: char) -> Result<Self, AttributeParsingError> {
        match value {
            '?' => Ok(Self::Q),
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            'E' => Ok(Self::E),
            _ => Err(ERROR),
        }
    }

    /// Tries to parse a `u8` as an [`EcoChar`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this byte into an [`EcoChar`].
    pub const fn from_ascii(value: u8) -> Result<Self, AttributeParsingError> {
        match value {
            b'?' => Ok(Self::Q),
            b'A' => Ok(Self::A),
            b'B' => Ok(Self::B),
            b'C' => Ok(Self::C),
            b'D' => Ok(Self::D),
            b'E' => Ok(Self::E),
            _ => Err(ERROR),
        }
    }
}

/// ECO code of an opening.
#[derive(Debug, Clone, Copy)]
pub struct Eco(pub EcoChar, pub RangedU8<0, 99>);

impl Eco {
    /// Tries to parse a `&str` as a [`RangedU8<0,99>`]
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into an [`Eco`]'s number.
    fn parse_eco_num(value: &str) -> Result<RangedU8<0, 99>, AttributeParsingError> {
        value.parse().map_err(|_| ERROR)
    }

    /// Tries to parse a `&str` as an [`Eco`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into an [`Eco`].
    pub fn from_str(value: &str) -> Result<Self, AttributeParsingError> {
        let mut chars = value.chars();
        if let Some(char) = chars.next() {
            Ok(Self(
                EcoChar::from_char(char)?,
                Self::parse_eco_num(chars.as_str())?,
            ))
        } else {
            Err(ERROR)
        }
    }

    /// Tries to parse a `&[u8]` as an [`Eco`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this bytes slice into an [`Eco`].
    pub fn from_ascii(value: &[u8]) -> Result<Self, AttributeParsingError> {
        let mut iter = value.into_iter();
        if let Some(char) = iter.next() {
            Ok(Self(
                EcoChar::from_ascii(*char)?,
                Self::parse_eco_num(from_utf8(value).map_err(|_| ERROR)?)?,
            ))
        } else {
            Err(ERROR)
        }
    }
}

impl Default for Eco {
    fn default() -> Self {
        Self(EcoChar::default(), RangedU8::new_static::<0>())
    }
}

impl Display for Eco {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:02}", self.0.as_char(), self)
    }
}

crate::tattribute!(Eco, "<?|[A-E][00-99]>");
