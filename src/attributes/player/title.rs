//! Specification and parsing of an Title rating from a Lichess player.

use deranged::RangedU8;

use super::super::error::AttributeParsingError;

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
use mysql::prelude::FromValue;

/// All possible [`Title`]s, ensuring the format is exhaustive.
const ALL_TITLES: [&str; 16] = [
    BOT_STR, LM_STR, GM_STR, WGM_STR, IM_STR, WIM_STR, FM_STR, WFM_STR, CM_STR, WCM_STR, NM_STR,
    WNM_STR, GR_STR, MC_STR, MN_STR, M_STR,
];

/// ASCII string slice representing a bot.
const BOT_ASCII: &[u8] = BOT_STR.as_bytes();
/// ASCII string slice representing a Lichess master.
const LM_ASCII: &[u8] = LM_STR.as_bytes();
/// ASCII string slice representing a grandmaster.
const GM_ASCII: &[u8] = GM_STR.as_bytes();
/// ASCII string slice representing a woman grandmaster.
const WGM_ASCII: &[u8] = WGM_STR.as_bytes();
/// ASCII string slice representing an international master.
const IM_ASCII: &[u8] = IM_STR.as_bytes();
/// ASCII string slice representing a woman international master.
const WIM_ASCII: &[u8] = WIM_STR.as_bytes();
/// ASCII string slice representing a FIDE master.
const FM_ASCII: &[u8] = FM_STR.as_bytes();
/// ASCII string slice representing a woman FIDE master.
const WFM_ASCII: &[u8] = WFM_STR.as_bytes();
/// ASCII string slice representing a candidate master.
const CM_ASCII: &[u8] = CM_STR.as_bytes();
/// ASCII string slice representing a woman candidate master.
const WCM_ASCII: &[u8] = WCM_STR.as_bytes();
/// ASCII string slice representing a national master.
const NM_ASCII: &[u8] = NM_STR.as_bytes();
/// ASCII string slice representing a woman national master.
const WNM_ASCII: &[u8] = WNM_STR.as_bytes();
/// ASCII string slice representing a Russian grandmaster.
const GR_ASCII: &[u8] = GR_STR.as_bytes();
/// ASCII string slice representing a master of sport.
const MC_ASCII: &[u8] = MC_STR.as_bytes();
/// ASCII string slice representing the MN title.
const MN_ASCII: &[u8] = MN_STR.as_bytes();
/// ASCII string slice representing the M title.
const M_ASCII: &[u8] = M_STR.as_bytes();

/// UTF-8 string slice representing a bot.
const BOT_STR: &str = "BOT";
/// UTF-8 string slice representing a Lichess master.
const LM_STR: &str = "LM";
/// UTF-8 string slice representing a grandmaster.
const GM_STR: &str = "GM";
/// UTF-8 string slice representing a woman grandmaster.
const WGM_STR: &str = "WGM";
/// UTF-8 string slice representing an international master.
const IM_STR: &str = "IM";
/// UTF-8 string slice representing a woman international master.
const WIM_STR: &str = "WIM";
/// UTF-8 string slice representing a FIDE master.
const FM_STR: &str = "FM";
/// UTF-8 string slice representing a woman FIDE master.
const WFM_STR: &str = "WFM";
/// UTF-8 string slice representing a candidate master.
const CM_STR: &str = "CM";
/// UTF-8 string slice representing a woman candidate master.
const WCM_STR: &str = "WCM";
/// UTF-8 string slice representing a national master.
const NM_STR: &str = "NM";
/// UTF-8 string slice representing a woman national master.
const WNM_STR: &str = "WNM";
/// UTF-8 string slice representing a Russian grandmaster.
const GR_STR: &str = "ГР";
/// UTF-8 string slice representing a master of sport.
const MC_STR: &str = "MC";
/// UTF-8 string slice representing the MN title.
const MN_STR: &str = "MN";
/// UTF-8 string slice representing the M title.
const M_STR: &str = "M";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    any(feature = "time-mysql", feature = "chrono-mysql"),
    derive(FromValue)
)]
#[repr(u8)]
pub enum Title {
    BOT = 1,
    LM,
    GM,
    WGM,
    IM,
    WIM,
    FM,
    WFM,
    CM,
    WCM,
    NM,
    WNM,
    GR,
    MC,
    MN,
    M,
}

impl Title {
    /// Retrieves the representation of this [`Title`] as a [`u8`], a value between 0 and 15.
    pub const fn as_u8(&self) -> u8 {
        (*self as u8) - 1
    }

    /// Retrieves the representation of this [`Title`] as a [`RangedU8`], a value between 0 and 15.
    pub const fn as_ranged(&self) -> RangedU8<0, 15> {
        RangedU8::new((*self as u8) - 1).expect("There are only 16 enum variants, this must work.")
    }

    /// Retrieves the representation of this [`Title`] as a `&'static str`.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BOT => BOT_STR,
            Self::LM => LM_STR,
            Self::GM => GM_STR,
            Self::WGM => WGM_STR,
            Self::IM => IM_STR,
            Self::WIM => WIM_STR,
            Self::FM => FM_STR,
            Self::WFM => WFM_STR,
            Self::CM => CM_STR,
            Self::WCM => WCM_STR,
            Self::NM => NM_STR,
            Self::WNM => WNM_STR,
            Self::GR => GR_STR,
            Self::MC => MC_STR,
            Self::MN => MN_STR,
            Self::M => M_STR,
        }
    }

    /// Retrieves the representation of this [`Title`] as a `&'static [u8]`.
    pub const fn as_ascii(&self) -> &'static [u8] {
        match self {
            Self::BOT => BOT_ASCII,
            Self::LM => LM_ASCII,
            Self::GM => GM_ASCII,
            Self::WGM => WGM_ASCII,
            Self::IM => IM_ASCII,
            Self::WIM => WIM_ASCII,
            Self::FM => FM_ASCII,
            Self::WFM => WFM_ASCII,
            Self::CM => CM_ASCII,
            Self::WCM => WCM_ASCII,
            Self::NM => NM_ASCII,
            Self::WNM => WNM_ASCII,
            Self::GR => GR_ASCII,
            Self::MC => MC_ASCII,
            Self::MN => MN_ASCII,
            Self::M => M_ASCII,
        }
    }

    /// Tries to parse a `&str` as a [`Title`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this string slice into a [`Title`].
    pub const fn from_str(value: &str) -> Result<Self, AttributeParsingError> {
        Self::from_ascii(value.as_bytes())
    }

    /// Tries to parse a `&[u8]` as a [`Title`].
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if it's not possible to parse this bytes slice into a [`Title`].
    pub const fn from_ascii(value: &[u8]) -> Result<Self, AttributeParsingError> {
        match value {
            BOT_ASCII => Ok(Self::BOT),
            LM_ASCII => Ok(Self::LM),
            GM_ASCII => Ok(Self::GM),
            WGM_ASCII => Ok(Self::WGM),
            IM_ASCII => Ok(Self::IM),
            WIM_ASCII => Ok(Self::WIM),
            FM_ASCII => Ok(Self::FM),
            WFM_ASCII => Ok(Self::WFM),
            CM_ASCII => Ok(Self::CM),
            WCM_ASCII => Ok(Self::WCM),
            NM_ASCII => Ok(Self::NM),
            WNM_ASCII => Ok(Self::WNM),
            GR_ASCII => Ok(Self::GR),
            MC_ASCII => Ok(Self::MC),
            MN_ASCII => Ok(Self::MN),
            M_ASCII => Ok(Self::M),
            _ => Err(ERROR),
        }
    }
}

crate::eattribute!(Title, &ALL_TITLES);
