//! Checker visitor for a PGN reader. Used for checking if the data is consistent with the predictions.

use std::str::from_utf8;

use chrono::{NaiveDate, NaiveTime};
use lichess::{
    attributes::{
        Eco, Elo, Eval, Opening, Result as ResultAttr, RuleSet, Termination, TimeControl, Title,
    },
    constants::{
        comments::{CLK, EVAL},
        headers::{
            BLACK, BLACK_ELO, BLACK_RATING_DIFF, BLACK_TITLE, DATE, ECO, EVENT, OPENING, RESULT,
            ROUND, SITE, TERMINATION, TIME_CONTROL, UTC_DATE, UTC_TIME, WHITE, WHITE_ELO,
            WHITE_RATING_DIFF, WHITE_TITLE,
        },
    },
    //data::Game,
};

use log::{error, info};
use pgn_reader::{Nag, RawComment, RawHeader, SanPlus, Visitor};
use shakmaty::Outcome;

use crate::visitors::comment_iterator::CommentIterator;

/// Checker visitor for a PGN reader.
#[derive(Debug, Default)]
pub struct Checker {
    /// Number of games processed (or the current game during a visit).
    pub games: usize,
    /// Number of moves processed in the game.
    moves: usize,
    /// Whether or not the checking has yield any errors.
    pub has_errors: bool,

    /// Current result of the visit.
    current_result: ResultAttr,
    /// Current termination of the visit.
    current_termination: Termination,
    /// Current date of the visit, either UTCDate or Date, whichever game first.
    current_date: Option<Vec<u8>>,

    /// Whether a `Site` has been seen in the current game or not.
    site: bool,
    /// Whether a [`TimeControl`] has been seen in the current game or not.
    time_control: bool,
    /// Whether a [`ResultAttr`] has been seen in the current game or not.
    result: bool,
    /// Whether a [`Termination`] has been seen in the current game or not.
    termination: bool,
    /// Whether a [`Date`] has been seen in the current game or not.
    date: bool,
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
    /// Whether a `Round` has been seen in the current game or not.
    round: bool,
    /// Whether a `White` player has been seen in the current game or not.
    white: bool,
    /// Whether a `White` player's `Elo` has been seen in the current game or not.
    white_elo: bool,
    /// Whether a `White` player's `RatingDiff` has been seen in the current game or not.
    white_rating_diff: bool,
    /// Whether a `White` player's [`Title`] has been seen in the current game or not.
    white_title: bool,
    /// Whether a `Black` player has been seen in the current game or not.
    black: bool,
    /// Whether a `Black` player's `Elo` has been seen in the current game or not.
    black_elo: bool,
    /// Whether a `Black` player's `RatingDiff` has been seen in the current game or not.
    black_rating_diff: bool,
    /// Whether a `Black` player's [`Title`] has been seen in the current game or not.
    black_title: bool,
}

impl Checker {
    /// Starts checking a new game, adding one to the game counter and resetting all other fields.
    pub fn new_game(&mut self) {
        self.games += 1;
        self.moves = 0;

        self.current_result = ResultAttr::Null;
        self.current_termination = Termination::Unterminated;
        self.current_date = None;

        self.site = false;
        self.time_control = false;
        self.result = false;
        self.termination = false;
        self.date = false;
        self.utc_date = false;
        self.utc_time = false;
        self.opening = false;
        self.eco = false;
        self.event = false;
        self.round = false;
        self.white = false;
        self.white_elo = false;
        self.white_rating_diff = false;
        self.white_title = false;
        self.black = false;
        self.black_elo = false;
        self.black_rating_diff = false;
        self.black_title = false;
    }

    pub fn check_comment(&mut self, key: &[u8], value: &[u8]) {
        let value = match from_utf8(value) {
            Ok(str) => str,
            Err(_) => {
                let str = String::from_utf8_lossy(value);
                error!(
                    "{} - Invalid UTF-8 comment: {} <- {:?}",
                    self.games, str, value
                );
                return;
            }
        };
        match key {
            CLK => {
                if let Err(e) = NaiveTime::parse_from_str(value, "%H:%M:%S") {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {} ({})", game_id + 1, e, value);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{} ({})", e, value);
                }
            }
            EVAL => {
                if let Err(e) = Eval::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {} ({})", game_id + 1, e, value);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{} ({})", e, value);
                }
            }
            key => {
                #[cfg(feature = "stats")]
                eprintln!(
                    "{} - New comment found: {} <- {:?}",
                    game_id + 1,
                    String::from_utf8_lossy(key),
                    key
                );
                #[cfg(not(feature = "stats"))]
                eprintln!(
                    "New comment found: {} <- {:?}",
                    String::from_utf8_lossy(key),
                    key
                );
            }
        }
    }

    pub fn check_game(
        &mut self,
        #[cfg(feature = "stats")] game_id: usize,
        #[cfg(feature = "raw-data")] _game: &Game,
    ) {
        if !self.site {
            error!("{} - Site is null.", self.games);
        }
        if !self.time_control {
            error!("{} - TimeControl is null.", self.games);
        }
        if !self.result {
            error!("{} - Result is null.", self.games);
        }
        if !self.termination {
            error!("{} - Termination is null.", self.games);
        }
        #[cfg(feature = "raw-data")]
        if (_game.termination == "Unterminated") & !(_game.result == "*") {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Unterminated with result.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Abandoned or Unterminated with result.");
        }
        if !self.date {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Date is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Date is null.");
        }
        if !self.utc_time {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Time is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Time is null.");
        }
        if !self.opening {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Opening is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Opening is null.");
        }
        if !self.eco {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Eco is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Eco is null.");
        }
        if self.eco ^ self.opening {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Opening without Eco / Eco without Opening.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Opening without Eco / Eco without Opening..");
        }
        if !self.event {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Event is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Event is null.");
        }
        if !self.white {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - White is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("White is null.");
        }
        if !self.white_elo {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - WhiteElo is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("WhiteElo is null.");
        }
        if !self.black {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Black is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Black is null.");
        }
        if !self.black_elo {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - BlackElo is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("BlackElo is null.");
        }
        *self = Self::default();
    }

    pub fn check_header(
        &mut self,
        #[cfg(feature = "stats")] game_id: usize,
        key: &[u8],
        value: &[u8],
    ) {
        let value = match from_utf8(value) {
            Ok(str) => str,
            Err(_) => {
                let str = String::from_utf8_lossy(value);
                #[cfg(feature = "stats")]
                panic!("{} - Invalid UTF-8: {} <- {:?}", game_id + 1, str, value);
                #[cfg(not(feature = "stats"))]
                eprintln!("Invalid UTF-8: {} <- {:?}", str, value);
                return;
            }
        };
        match key {
            SITE => self.site = true,
            TIME_CONTROL => {
                self.time_control = true;
                if let Err(e) = TimeControl::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            }
            RESULT => {
                self.result = true;
                if let Err(e) = ResultAttr::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            }
            TERMINATION => {
                self.termination = true;
                if let Err(e) = Termination::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            }
            DATE => {
                if let Some(date) = &self.current_date {
                    if String::from_utf8_lossy(date) != value {
                        #[cfg(feature = "stats")]
                        eprintln!("{} - UTCDate is different than Date", game_id + 1);
                        #[cfg(not(feature = "stats"))]
                        eprintln!("UTCDate is different than Date");
                    }
                } else {
                    self.current_date = Some(value.into());
                }
            }
            UTC_DATE => {
                self.date = true;
                if let Err(e) = NaiveDate::parse_from_str(value, "%Y.%m.%d") {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {} ({})", game_id + 1, e, value);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{} ({})", e, value);
                }
                if let Some(date) = &self.current_date {
                    if String::from_utf8_lossy(date) != value {
                        #[cfg(feature = "stats")]
                        eprintln!("{} - UTCDate is different than Date", game_id + 1);
                        #[cfg(not(feature = "stats"))]
                        eprintln!("UTCDate is different than Date");
                    }
                } else {
                    self.current_date = Some(value.into());
                }
            }
            UTC_TIME => {
                self.utc_time = true;
                if let Err(e) = NaiveTime::parse_from_str(value, "%H:%M:%S") {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {} ({})", game_id + 1, e, value);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{} ({})", e, value);
                }
            }
            OPENING => {
                self.opening = true;
                let _ = Opening(value.to_owned());
            }
            ECO => {
                self.eco = true;
                if let Err(e) = Eco::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            }
            EVENT => {
                self.event = true;
                let _ = RuleSet::from(value);
            }
            ROUND => {
                if value != "-" {
                    println!("{}", value)
                }
            }
            WHITE => self.white = true,
            WHITE_ELO => {
                self.white_elo = true;
                if let Err(e) = Elo::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            }
            WHITE_RATING_DIFF => {
                if let Err(e) = value.parse::<i16>() {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            }
            WHITE_TITLE => {
                if let Err(e) = Title::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            }
            BLACK => self.black = true,
            BLACK_ELO => {
                self.black_elo = true;
                if let Err(e) = Elo::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            }
            BLACK_RATING_DIFF => {
                if let Err(e) = value.parse::<i16>() {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            }
            BLACK_TITLE => {
                if let Err(e) = Title::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            }
            key => {
                #[cfg(feature = "stats")]
                eprintln!(
                    "{} - New header found: {} <- {:?}",
                    game_id + 1,
                    String::from_utf8_lossy(key),
                    key
                );
                #[cfg(not(feature = "stats"))]
                eprintln!(
                    "New header found: {} <- {:?}",
                    String::from_utf8_lossy(key),
                    key
                );
            }
        }
    }
}

impl Visitor for Checker {
    type Result = ();

    fn begin_game(&mut self) {
        self.new_game();
        if self.games % 1000000 == 0 {
            info!("Processed {} games.", self.games);
        }
    }

    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        self.check_header(_key, _value.0);
    }

    fn san(&mut self, _san: SanPlus) {
        #[cfg(feature = "data")]
        if self.data.is_move_valid() {
            self.serializer.write_move(&self.data.r#move);
        }
        #[cfg(feature = "data")]
        self.data.new_move(_san);
        #[cfg(feature = "stats")]
        self.stats.san(NULL_SAN);
    }

    fn nag(&mut self, _nag: Nag) {
        #[cfg(feature = "data")]
        self.data.add_nag(_nag);
        #[cfg(feature = "stats")]
        self.stats.nag(NULL_NAG);
    }

    fn comment(&mut self, _comment: RawComment<'_>) {
        let comments = CommentIterator::new(_comment.0);
        for (key, value) in comments {
            #[cfg(feature = "check")]
            Checker::check_comment(
                #[cfg(feature = "stats")]
                self.stats.games,
                key,
                value,
            );
            #[cfg(feature = "collection")]
            self.collector.collect_comment(key);
            #[cfg(feature = "data")]
            self.data.r#move.set(key, value);
        }
        #[cfg(feature = "stats")]
        self.stats.comment(RawComment(&[]));
    }

    fn end_variation(&mut self) {
        #[cfg(feature = "stats")]
        self.stats.end_variation();
    }

    fn outcome(&mut self, _outcome: Option<Outcome>) {
        #[cfg(feature = "stats")]
        self.stats.outcome(None);
    }

    fn end_game(&mut self) {
        self.check_game(
            #[cfg(feature = "raw-data")]
            &self.data.game,
        );
    }
}
