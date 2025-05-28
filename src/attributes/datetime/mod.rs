//! Specification and parsing of dates and times.

#[cfg(all(feature = "chrono", feature = "time"))]
compile_error!("The features chrono and time cannot be enabled at the same time.");

#[cfg(not(any(feature = "chrono", feature = "time")))]
compile_error!("At least one of the features chrono and time must be enabled.");

use std::fmt::Display;

use super::AttributeFormat;

pub mod clk;
pub mod date;
pub mod utc_date;
pub mod utc_time;

#[cfg(feature = "chrono")]
mod chrono {
    use chrono::{NaiveDate as CDate, NaiveDateTime as CDateTime, NaiveTime as CTime, ParseError};

    const CDATE_FORMAT: &str = "%Y.%m.%d";
    const CTIME_FORMAT: &str = "%H:%H:%S";

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct Date(pub CDate);

    impl Date {
        /// Parses the value from a string.
        ///
        /// # Errors
        /// Will return [`ParseError`] if it's not possible to parse the value from the given string (parse error).
        pub fn parse(input: &str) -> Result<Self, ParseError> {
            CDate::parse_from_str(input, CDATE_FORMAT).map(|date| Self(date))
        }
    }

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct Time(pub CTime);

    impl Time {
        /// Parses the value from a string.
        ///
        /// # Errors
        /// Will return [`ParseError`] if it's not possible to parse the value from the given string (parse error).
        pub fn parse(input: &str) -> Result<Self, ParseError> {
            CTime::parse_from_str(input, CTIME_FORMAT).map(|time| Self(time))
        }
    }

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct Datetime(pub CDateTime);

    impl Datetime {
        /// Creates a new value from a [`Date`] and a [`Time`].
        pub fn new(date: Date, time: Time) -> Self {
            Self(CDateTime::new(date.0, time.0))
        }
    }
}

#[cfg(feature = "time")]
mod time {
    use time::{
        Date as TDate, PrimitiveDateTime as TDateTime, Time as TTime, error::Parse,
        format_description::BorrowedFormatItem, macros::format_description,
    };

    const TDATE_FORMAT: &[BorrowedFormatItem<'static>] =
        format_description!("[year].[month].[day]");
    const TTIME_FORMAT: &[BorrowedFormatItem<'static>] =
        format_description!("[hour]:[minute]:[second]");

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Date(pub TDate);

    impl Default for Date {
        fn default() -> Self {
            Self(TDate::from_julian_day(0).expect("The julian day 0 must exist."))
        }
    }

    impl Date {
        /// Parses the value from a string.
        ///
        /// # Errors
        /// Will return [`Parse`] if it's not possible to parse the value from the given string (parse error).
        pub fn parse(input: &str) -> Result<Self, Parse> {
            TDate::parse(input, TDATE_FORMAT).map(|date| Self(date))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Time(pub TTime);

    impl Default for Time {
        fn default() -> Self {
            Self(TTime::from_hms(0, 0, 0).expect("The hour 00:00:00 must exist."))
        }
    }

    impl Time {
        /// Parses the value from a string.
        ///
        /// # Errors
        /// Will return [`Parse`] if it's not possible to parse the value from the given string (parse error).
        pub fn parse(input: &str) -> Result<Self, Parse> {
            TTime::parse(input, TTIME_FORMAT).map(|time| Self(time))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Datetime(pub TDateTime);

    impl Default for Datetime {
        fn default() -> Self {
            Self::new(Date::default(), Time::default())
        }
    }

    impl Datetime {
        /// Creates a new value from a [`Date`] and a [`Time`].
        pub fn new(date: Date, time: Time) -> Self {
            Self(TDateTime::new(date.0, time.0))
        }
    }
}

#[cfg(feature = "chrono")]
pub use self::chrono::{Date, Datetime, Time};

#[cfg(feature = "time")]
pub use self::time::{Date, Datetime, Time};

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub const DATE_FORMAT: AttributeFormat = AttributeFormat::Str("{:YYYY}.{:MM}.{:DD}");
pub const TIME_FORMAT: AttributeFormat = AttributeFormat::Str("{:hh}:{:mm}:{:ss}");
