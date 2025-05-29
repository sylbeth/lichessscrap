//! Module containing all the different PGN reader visitors that this application uses and the error macros they use.

pub mod checkcollect;
pub mod comment_iterator;
pub mod serializer;
pub mod stats;

/// An error without more context than the game it happened at.
#[macro_export]
macro_rules! loneerror {
    ($str:literal, $self:ident) => {
        log::error!(concat!("{} - ", $str), $self.games);
        $self.has_errors = true;
    };
}

/// An error about a header of a game that is null.
#[macro_export]
macro_rules! nullerror {
    ($str:literal, $self:ident) => {
        log::error!(concat!("{} - ", $str, " is null"), $self.games);
        $self.has_errors = true;
    };
}

/// An error with context and the game it happened at.
#[macro_export]
macro_rules! valuederror {
    ($self:ident, $error:ident) => {
        log::error!("{} - {}", $self.games, $error);
        $self.has_errors = true;
    };
}
