use std::str::from_utf8;

use chrono::{NaiveDate, NaiveTime};
use lichess::{
    attributes::{Eco, Elo, Eval, Opening, ResultAttr, RuleSet, Termination, TimeControl, Title},
    constants::{
        comments::{CLK, EVAL},
        headers::{
            BLACK, BLACK_ELO, BLACK_RATING_DIFF, BLACK_TITLE, DATE, ECO, EVENT, OPENING, RESULT,
            ROUND, SITE, TERMINATION, TIME_CONTROL, UTC_DATE, UTC_TIME, WHITE, WHITE_ELO,
            WHITE_RATING_DIFF, WHITE_TITLE,
        },
    },
};

#[cfg(feature = "raw-data")]
use lichess::data::Game;

#[derive(Debug, Default)]
pub struct Checker {
    current_date: Option<String>,
    site: bool,
    time_control: bool,
    result: bool,
    termination: bool,
    date: bool,
    time: bool,
    opening: bool,
    eco: bool,
    event: bool,
    white: bool,
    white_elo: bool,
    black: bool,
    black_elo: bool,
}

impl Checker {
    pub fn check_comment(#[cfg(feature = "stats")] game_id: usize, key: &[u8], value: &[u8]) {
        let value = match from_utf8(value) {
            Ok(str) => str,
            Err(_) => {
                let str = String::from_utf8_lossy(value);
                #[cfg(feature = "stats")]
                panic!("{} - Invalid UTF-8: {} <- {:?}", game_id + 1, str, value);
                #[cfg(not(feature = "stats"))]
                eprintln!("Invalid UTF-8: {} <- {:?}", str, value);
            }
        };
        match key {
            CLK => if let Err(e) = NaiveTime::parse_from_str(value, "%H:%M:%S") {
                #[cfg(feature = "stats")]
                eprintln!("{} - {} ({})", game_id + 1, e, value);
                #[cfg(not(feature = "stats"))]
                eprintln!("{} ({})", e, value);
            },
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
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Site is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Site is null.");
        }
        if !self.time_control {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - TimeControl is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("TimeControl is null.");
        }
        if !self.result {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Result is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Result is null.");
        }
        if !self.termination {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Termination is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Termination is null.");
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
        if !self.time {
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
            eprintln!("{game_id} - ECO is null.");
            #[cfg(not(feature = "stats"))]
            eprintln!("ECO is null.");
        }
        if self.eco ^ self.opening {
            #[cfg(feature = "stats")]
            eprintln!("{game_id} - Opening without ECO / ECO without Opening.");
            #[cfg(not(feature = "stats"))]
            eprintln!("Opening without ECO / ECO without Opening..");
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
                    if date != value {
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
                    if date != value {
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
                self.time = true;
                if let Err(e) = NaiveTime::parse_from_str(value, "%H:%M:%S") {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {} ({})", game_id + 1, e, value);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{} ({})", e, value);
                }
            },
            OPENING => {
                self.opening = true;
                let _ = Opening::from(value);
            },
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
            },
            ROUND => if value != "-" {println!("{}", value)},
            WHITE => self.white = true,
            WHITE_ELO => {
                self.white_elo = true;
                if let Err(e) = Elo::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            },
            WHITE_RATING_DIFF => if let Err(e) = value.parse::<i16>() {
                #[cfg(feature = "stats")]
                eprintln!("{} - {}", game_id + 1, e);
                #[cfg(not(feature = "stats"))]
                eprintln!("{}", e);
            },
            WHITE_TITLE => if let Err(e) = Title::try_from(value) {
                #[cfg(feature = "stats")]
                eprintln!("{} - {}", game_id + 1, e);
                #[cfg(not(feature = "stats"))]
                eprintln!("{}", e);
            },
            BLACK => self.black = true,
            BLACK_ELO => {
                self.black_elo = true;
                if let Err(e) = Elo::try_from(value) {
                    #[cfg(feature = "stats")]
                    eprintln!("{} - {}", game_id + 1, e);
                    #[cfg(not(feature = "stats"))]
                    eprintln!("{}", e);
                }
            },
            BLACK_RATING_DIFF => if let Err(e) = value.parse::<i16>() {
                #[cfg(feature = "stats")]
                eprintln!("{} - {}", game_id + 1, e);
                #[cfg(not(feature = "stats"))]
                eprintln!("{}", e);
            },
            BLACK_TITLE => if let Err(e) = Title::try_from(value) {
                #[cfg(feature = "stats")]
                eprintln!("{} - {}", game_id + 1, e);
                #[cfg(not(feature = "stats"))]
                eprintln!("{}", e);
            },
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
