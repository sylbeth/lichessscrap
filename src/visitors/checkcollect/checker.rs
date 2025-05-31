//! Checker visitor for a PGN reader. Used for checking if the data is consistent with the predictions.

use std::str::from_utf8;

use log::{error, info};
use pgn_reader::{RawComment, RawHeader, Visitor};
use shakmaty::Outcome;

#[cfg(feature = "full-check")]
use pgn_reader::SanPlus;
#[cfg(feature = "full-check")]
use shakmaty::{Chess, Position};

use crate::{loneerror, nullerror, valuederror, visitors::comment_iterator::CommentIterator};
use lichess::{
    attributes::{
        Clk, Date, Eco, Elo, Eval, Opening, Player, Result as ResultAttr, RuleSet, Termination,
        TimeControl, Title, UTCDate, UTCTime, attribute::StringAttribute,
    },
    constants::{
        comments::{CLK, EVAL},
        headers::{
            BLACK, BLACK_ELO, BLACK_RATING_DIFF, BLACK_TITLE, DATE, ECO, EVENT, OPENING, RESULT,
            ROUND, SITE, TERMINATION, TIME_CONTROL, UTC_DATE, UTC_TIME, WHITE, WHITE_ELO,
            WHITE_RATING_DIFF, WHITE_TITLE,
        },
    },
};

/// Checker visitor for a PGN reader.
#[derive(Debug, Default)]
pub struct Checker {
    /// Number of games processed (or the current game during a visit).
    pub games: usize,
    /// Number of moves processed in the game.
    #[cfg(feature = "full-check")]
    moves: usize,
    /// Whether or not the checking has yield any errors.
    pub has_errors: bool,

    /// The board for checking move validity.
    #[cfg(feature = "full-check")]
    chess: Chess,

    /// Current result of the visit.
    current_result: ResultAttr,
    /// Current termination of the visit.
    current_termination: Termination,
    /// Current date of the visit, either UTCDate or Date, whichever came first.
    current_date: Option<lichess::attributes::datetime::Date>,
    /// Whether [`UTCDate`] or [`Date`] appear first.
    utc_first: bool,
    /// Current opening of the visit.
    current_opening: Opening,
    /// Current player of the visit.
    current_player: Player,

    /// Whether a `Site` has been seen in the current game or not.
    site: bool,
    /// Whether a [`TimeControl`] has been seen in the current game or not.
    time_control: bool,
    /// Whether a [`ResultAttr`] has been seen in the current game or not.
    result: bool,
    /// Whether a [`Termination`] has been seen in the current game or not.
    termination: bool,
    /*
    /// Whether a [`Date`] has been seen in the current game or not.
    date: bool,
    */
    /// Whether an `UTC`[`Date`] has been seen in the current game or not.
    utc_date: bool,
    /// Whether a `UTC`[`Time`] has been seen in the current game or not.
    utc_time: bool,
    /// Whether an [`Opening`] has been seen in the current game or not.
    opening: bool,
    /// Whether an [`Eco`] has been seen in the current game or not.
    eco: bool,
    /// Whether an `Event` has been seen in the current game or not.
    event: bool,
    /*
    /// Whether a `Round` has been seen in the current game or not.
    round: bool,
    */
    /// Whether a `Black` player has been seen in the current game or not.
    black: bool,
    /// Whether a `Black` player's `Elo` has been seen in the current game or not.
    black_elo: bool,
    /*
    /// Whether a `Black` player's `RatingDiff` has been seen in the current game or not.
    black_rating_diff: bool,
    */
    /*
    /// Whether a `Black` player's [`Title`] has been seen in the current game or not.
    black_title: bool,
    */
    /// Whether a `White` player has been seen in the current game or not.
    white: bool,
    /// Whether a `White` player's `Elo` has been seen in the current game or not.
    white_elo: bool,
    /*
    /// Whether a `White` player's `RatingDiff` has been seen in the current game or not.
    white_rating_diff: bool,
    */
    /*
    /// Whether a `White` player's [`Title`] has been seen in the current game or not.
    white_title: bool,
    */
}

impl Checker {
    /// Starts checking a new game, adding one to the game counter and resetting all other fields.
    pub fn new_game(&mut self) {
        self.games += 1;

        self.current_result = ResultAttr::Null;
        self.current_termination = Termination::Unterminated;
        self.current_date = None;
        self.current_opening.clear();
        self.current_player.clear();

        #[cfg(feature = "full-check")]
        {
            self.moves = 0;
            self.chess = Chess::new();
        }

        self.site = false;
        self.time_control = false;
        self.result = false;
        self.termination = false;
        //self.date = false;
        self.utc_date = false;
        self.utc_time = false;
        self.opening = false;
        self.eco = false;
        self.event = false;
        //self.round = false;
        self.black = false;
        self.black_elo = false;
        //self.black_rating_diff = false;
        //self.black_title = false;
        self.white = false;
        self.white_elo = false;
        //self.white_rating_diff = false;
        //self.white_title = false;
    }

    /// Checks whether a commment is valid or not.
    pub fn check_comment(&mut self, key: &[u8], value: &[u8]) {
        match key {
            CLK => {
                if let Err(e) = Clk::try_from(value) {
                    valuederror!(self, e);
                }
            }
            EVAL => {
                if let Err(e) = Eval::try_from(value) {
                    valuederror!(self, e);
                }
            }
            key => {
                error!(
                    "{} - New comment found: {} <- {:?}",
                    self.games,
                    String::from_utf8_lossy(key),
                    key
                );
                self.has_errors = true;
            }
        }
    }

    /// Checks whether the last game read is valid or not.
    pub fn check_game(&mut self) {
        if !self.site {
            nullerror!("Site", self);
        }
        if !self.time_control {
            nullerror!("Time Control", self);
        }
        if !self.result {
            nullerror!("Result", self);
        }
        if !self.termination {
            nullerror!("Termination", self);
        }
        if (self.current_termination == Termination::Unterminated)
            & (self.current_result != ResultAttr::Null)
        {
            loneerror!("Unterminated with result", self);
        }
        /*
        if !self.date {
            nullerror!("Date", self);
        }
        */
        if !self.utc_date {
            nullerror!("UTC Date", self);
        }
        if !self.utc_time {
            nullerror!("UTC Time", self);
        }
        if !self.opening {
            nullerror!("Opening", self);
        }
        if !self.eco {
            nullerror!("ECO", self);
        }
        if self.eco ^ self.opening {
            loneerror!("Opening without ECO / ECO without Opening", self);
        }
        if !self.event {
            nullerror!("Ruleset", self);
        }
        /*
        if !self.round {
            nullerror!("Round", self);
        }
        */
        if !self.black {
            nullerror!("Black player", self);
        }
        if !self.black_elo {
            nullerror!("Black player's elo", self);
        }
        /*
        if !self.black_rating_diff {
            nullerror!("Black player's rating diff", self);
        }
        */
        /*
        if !self.black_title {
            nullerror!("Black player's title", self);
        }
        */
        if !self.white {
            nullerror!("White player", self);
        }
        if !self.white_elo {
            nullerror!("White player's elo", self);
        }
        /*
        if !self.white_rating_diff {
            nullerror!("White player's rating diff", self);
        }
        */
        /*
        if !self.white_title {
            nullerror!("White player's title", self);
        }
        */
    }

    /// Checks whether a header is valid or not.
    pub fn check_header(&mut self, key: &[u8], value: &[u8]) {
        match key {
            SITE => {
                self.site = true;
                if from_utf8(value).is_err() {
                    error!(
                        "{} - Site is not UTF-8: {} <- {:?}",
                        self.games,
                        String::from_utf8_lossy(key),
                        key
                    );
                    self.has_errors = true;
                }
            }
            TIME_CONTROL => {
                self.time_control = true;
                if let Err(e) = TimeControl::try_from(value) {
                    valuederror!(self, e);
                }
            }
            RESULT => {
                self.result = true;
                match ResultAttr::try_from(value) {
                    Ok(result) => self.current_result = result,
                    Err(e) => {
                        valuederror!(self, e);
                    }
                }
            }
            TERMINATION => {
                self.termination = true;
                match Termination::try_from(value) {
                    Ok(termination) => self.current_termination = termination,
                    Err(e) => {
                        valuederror!(self, e);
                    }
                }
            }
            DATE => {
                //self.date = true;
                match (Date::try_from(value), self.current_date) {
                    (Ok(date), Some(utc_date)) => {
                        if date.0 != utc_date {
                            error!(
                                "{} - Date ({date}) is different than UTCDate ({utc_date})",
                                self.games
                            );
                            self.has_errors = true;
                        }
                    }
                    (Ok(date), None) => {
                        self.current_date = Some(date.0);
                        if self.utc_first {
                            info!("Date appeared first.");
                            self.utc_first = false;
                        }
                    }
                    (Err(e), _) => {
                        valuederror!(self, e);
                    }
                }
            }
            UTC_DATE => {
                self.utc_date = true;
                match (UTCDate::try_from(value), self.current_date) {
                    (Ok(utc_date), Some(date)) => {
                        if utc_date.0 != date {
                            error!(
                                "{} - UTC Date ({utc_date}) is different than Date ({date})",
                                self.games
                            );
                            self.has_errors = true;
                        }
                    }
                    (Ok(utc_date), None) => {
                        self.current_date = Some(utc_date.0);
                        if !self.utc_first {
                            info!("UTCDate appeared first.");
                            self.utc_first = true;
                        }
                    }
                    (Err(e), _) => {
                        valuederror!(self, e);
                    }
                }
            }
            UTC_TIME => {
                self.utc_time = true;
                if let Err(e) = UTCTime::try_from(value) {
                    valuederror!(self, e);
                }
            }
            OPENING => {
                self.opening = true;
                if let Err(e) = self.current_opening.fill_ascii(value) {
                    valuederror!(self, e);
                }
            }
            ECO => {
                self.eco = true;
                if let Err(e) = Eco::try_from(value) {
                    valuederror!(self, e);
                }
            }
            EVENT => {
                self.event = true;
                if let Err(e) = RuleSet::try_from(value) {
                    valuederror!(self, e);
                }
            }
            ROUND => {
                //self.round = true;
                if value != b"-" {
                    loneerror!("There is a round different from \"-\"", self);
                }
            }
            BLACK => {
                self.black = true;
                if let Err(e) = self.current_player.fill_ascii(value) {
                    valuederror!(self, e);
                }
            }
            BLACK_ELO => {
                self.black_elo = true;
                if let Err(e) = Elo::try_from(value) {
                    valuederror!(self, e);
                }
            }
            BLACK_RATING_DIFF => {
                //self.black_rating_diff = true;
                match from_utf8(value) {
                    Err(_) => {
                        error!(
                            "{} - Black rating diff is not UTF-8: {} <- {:?}",
                            self.games,
                            String::from_utf8_lossy(value),
                            value
                        );
                        self.has_errors = true;
                    }
                    Ok(value) => {
                        if value.parse::<i16>().is_err() {
                            error!(
                                "{} - Black rating diff is not a valid signed integer: {}",
                                self.games, value,
                            );
                            self.has_errors = true;
                        }
                    }
                }
            }
            BLACK_TITLE => {
                //self.black_title = true;
                if let Err(e) = Title::try_from(value) {
                    valuederror!(self, e);
                }
            }
            WHITE => {
                self.white = true;
                if let Err(e) = self.current_player.fill_ascii(value) {
                    valuederror!(self, e);
                }
            }
            WHITE_ELO => {
                self.white_elo = true;
                if let Err(e) = Elo::try_from(value) {
                    valuederror!(self, e);
                }
            }
            WHITE_RATING_DIFF => {
                //self.white_rating_diff = true;
                match from_utf8(value) {
                    Err(_) => {
                        error!(
                            "{} - White rating diff is not UTF-8: {} <- {:?}",
                            self.games,
                            String::from_utf8_lossy(value),
                            value
                        );
                        self.has_errors = true;
                    }
                    Ok(value) => {
                        if value.parse::<i16>().is_err() {
                            error!(
                                "{} - White rating diff is not a valid signed integer: {}",
                                self.games, value,
                            );
                            self.has_errors = true;
                        }
                    }
                }
            }
            WHITE_TITLE => {
                //self.white_title = true;
                if let Err(e) = Title::try_from(value) {
                    valuederror!(self, e);
                }
            }
            key => {
                error!(
                    "{} - New header found: {} <- {:?}",
                    self.games,
                    String::from_utf8_lossy(key),
                    key
                );
                self.has_errors = true;
            }
        }
    }
}

impl Visitor for Checker {
    type Result = ();

    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        self.check_header(_key, _value.0);
    }

    #[cfg(feature = "full-check")]
    fn san(&mut self, _san: SanPlus) {
        self.moves += 1;
        if let Ok(mv) = _san.san.to_move(&self.chess) {
            self.chess.play_unchecked(&mv);
        } else {
            error!(
                "{}.{} - Invalid SAN move played: {}",
                self.games, self.moves, _san,
            );
            self.has_errors = true;
        }
    }

    fn comment(&mut self, _comment: RawComment<'_>) {
        for (key, value) in CommentIterator::new(_comment.0) {
            self.check_comment(key, value);
        }
    }

    fn outcome(&mut self, _outcome: Option<Outcome>) {
        if self.current_result != ResultAttr::from(_outcome) {
            loneerror!("The outcome is different to the given result", self);
        }
    }

    fn end_game(&mut self) {
        self.check_game();
        self.new_game();
        if self.games % 1000000 == 0 {
            info!("Checked {} games.", self.games);
        }
    }
}
