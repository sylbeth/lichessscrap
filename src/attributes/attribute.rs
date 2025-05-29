//! Module with the macros and traits to generate common behaviour for attributes.

/// Generates the common implementations for an enum attribute.
#[macro_export]
macro_rules! eattribute {
    ($struct:ident, $array:expr) => {
        crate::attribute_fmt!($struct, $array);
        crate::attribute_err!($struct);

        impl TryFrom<&[u8]> for $struct {
            type Error = crate::attributes::error::ValuedAttributeParsingError;

            fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
                Self::from_ascii(value).map_err(|e| {
                    crate::attributes::error::ValuedAttributeParsingError::from_inner(
                        e,
                        value.to_owned(),
                    )
                })
            }
        }

        impl TryFrom<&str> for $struct {
            type Error = crate::attributes::error::ValuedAttributeParsingError;

            fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
                Self::from_str(value).map_err(|e| {
                    crate::attributes::error::ValuedAttributeParsingError::from_inner_utf8(
                        e,
                        value.to_owned(),
                    )
                })
            }
        }

        impl std::str::FromStr for $struct {
            type Err = crate::attributes::error::ValuedAttributeParsingError;

            fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
                Self::from_str(value).map_err(|e| {
                    crate::attributes::error::ValuedAttributeParsingError::from_inner_utf8(
                        e,
                        value.to_owned(),
                    )
                })
            }
        }

        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.as_str().fmt(f)
            }
        }

        impl AsRef<str> for $struct {
            fn as_ref(&self) -> &'static str {
                self.as_str()
            }
        }

        impl AsRef<[u8]> for $struct {
            fn as_ref(&self) -> &'static [u8] {
                self.as_ascii()
            }
        }
    };
}

/// Generates the common implementations for a struct attribute.
#[macro_export]
macro_rules! tattribute {
    ($struct:ident, $str:expr) => {
        crate::attribute_fmt!($struct, $str);
        crate::attribute_err!($struct);

        impl TryFrom<&[u8]> for $struct {
            type Error = crate::attributes::error::ValuedAttributeParsingError;

            fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
                Self::from_ascii(value).map_err(|e| {
                    crate::attributes::error::ValuedAttributeParsingError::from_inner(
                        e,
                        value.to_owned(),
                    )
                })
            }
        }

        impl TryFrom<&str> for $struct {
            type Error = crate::attributes::error::ValuedAttributeParsingError;

            fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
                Self::from_str(value).map_err(|e| {
                    crate::attributes::error::ValuedAttributeParsingError::from_inner_utf8(
                        e,
                        value.to_owned(),
                    )
                })
            }
        }

        impl std::str::FromStr for $struct {
            type Err = crate::attributes::error::ValuedAttributeParsingError;

            fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
                Self::from_str(value).map_err(|e| {
                    crate::attributes::error::ValuedAttributeParsingError::from_inner_utf8(
                        e,
                        value.to_owned(),
                    )
                })
            }
        }
    };
}

/// Generates the common implementations for a stringy struct attribute.
#[macro_export]
macro_rules! sattribute {
    ($struct:ident) => {
        crate::attribute_fmt!($struct, "<?|{str}>");
        crate::attribute_err!($struct);

        #[doc = concat!(stringify!($struct), "of a Lichess game.")]
        #[derive(Debug, Default, Clone)]
        pub struct $struct(pub String);

        impl crate::attributes::attribute::StringAttribute for $struct {
            fn clear(&mut self) {
                self.0.clear();
            }

            fn fill(&mut self, value: &str) {
                if value != "?" {
                    self.0.push_str(value);
                }
            }

            fn fill_ascii(
                &mut self,
                value: &[u8],
            ) -> std::result::Result<(), crate::attributes::error::ValuedAttributeParsingError>
            {
                if value != b"?" {
                    self.0.push_str(std::str::from_utf8(value).map_err(|_| {
                        crate::attributes::error::ValuedAttributeParsingError::from_inner(
                            ERROR,
                            value.to_owned(),
                        )
                    })?)
                }
                Ok(())
            }
        }

        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

/// Generates the common implementations for a datetime struct attribute.
#[macro_export]
macro_rules! dtattribute {
    ($struct:ident, $kind:ident) => {
        crate::attribute_err!($struct);

        #[doc = concat!(stringify!($struct), "of a Lichess game.")]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub struct $struct(pub $kind);

        impl crate::attributes::attribute::DatetimeAttribute for $struct {
            fn parse(
                input: &str,
            ) -> std::result::Result<Self, crate::attributes::error::ValuedAttributeParsingError>
            {
                match $kind::parse(input) {
                    Ok(value) => Ok(Self(value)),
                    Err(_) => Err(
                        crate::attributes::error::ValuedAttributeParsingError::from_inner_utf8(
                            ERROR,
                            input.to_owned(),
                        ),
                    ),
                }
            }

            fn parse_ascii(
                input: &[u8],
            ) -> std::result::Result<Self, crate::attributes::error::ValuedAttributeParsingError>
            {
                Self::parse(std::str::from_utf8(input).map_err(|_| {
                    crate::attributes::error::ValuedAttributeParsingError::from_inner(
                        ERROR,
                        input.to_owned(),
                    )
                })?)
            }
        }

        impl TryFrom<&[u8]> for $struct {
            type Error = crate::attributes::error::ValuedAttributeParsingError;

            fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
                crate::attributes::attribute::DatetimeAttribute::parse_ascii(value)
            }
        }

        impl TryFrom<&str> for $struct {
            type Error = crate::attributes::error::ValuedAttributeParsingError;

            fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
                crate::attributes::attribute::DatetimeAttribute::parse(value)
            }
        }

        impl std::str::FromStr for $struct {
            type Err = crate::attributes::error::ValuedAttributeParsingError;

            fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
                crate::attributes::attribute::DatetimeAttribute::parse(value)
            }
        }

        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

/// Generates the attribute format for the attribute.
#[macro_export]
macro_rules! attribute_fmt {
    ($struct:ident, $str:literal) => {
        #[doc = concat!("Format that a Lichess [`", stringify!($struct), "`] has.")]
        pub const FORMAT: crate::attributes::AttributeFormat =
            crate::attributes::AttributeFormat::Str($str);
    };
    ($struct:ident, $array:expr) => {
        #[doc = concat!("Format that a Lichess [`", stringify!($struct), "`] has.")]
        pub const FORMAT: crate::attributes::AttributeFormat =
            crate::attributes::AttributeFormat::Enum($array);
    };
}

/// Generates the attribute error for the attribute.
#[macro_export]
macro_rules! attribute_err {
    ($kind:ident) => {
        #[doc = concat!("Error that parsing [`", stringify!($kind), "`] could generate.")]
        pub const ERROR: crate::attributes::error::AttributeParsingError =
            crate::attributes::error::AttributeParsingError::new(
                crate::attributes::AttributeKind::$kind,
            );
    };
}

/// Methods for a stringy struct attribute.
pub trait StringAttribute {
    /// Truncates the inner [`String`], removing all its contents.
    fn clear(&mut self);

    /// Fills the inner [`String`] with the value, unless it's `"?"`.
    fn fill(&mut self, value: &str);

    /// Fills the inner [`String`] with the value, unless it's `b"?"`.
    ///
    /// # Errors
    /// Will return [`ValuedAttributeParsingError`](crate::attributes::error::ValuedAttributeParsingError) if it's not possible to fill the inner [`String`] with the bytes slice (invalid_utf8).
    fn fill_ascii(
        &mut self,
        value: &[u8],
    ) -> std::result::Result<(), crate::attributes::error::ValuedAttributeParsingError>;
}

/// Methods for a datetime struct attribute.
pub trait DatetimeAttribute: Sized {
    /// Parses the value from a string.
    ///
    /// # Errors
    /// Will return [`ValuedAttributeParsingError`](crate::attributes::error::ValuedAttributeParsingError) if it's not possible to parse the value from the given string (parse error).
    fn parse(
        input: &str,
    ) -> std::result::Result<Self, crate::attributes::error::ValuedAttributeParsingError>;

    /// Parses the value from a bytes slice.
    ///
    /// # Errors
    /// Will return [`ValuedAttributeParsingError`](crate::attributes::error::ValuedAttributeParsingError) if it's not possible to parse the value from the given bytes slice (invalid_utf8 or parse error).
    fn parse_ascii(
        input: &[u8],
    ) -> std::result::Result<Self, crate::attributes::error::ValuedAttributeParsingError>;
}
