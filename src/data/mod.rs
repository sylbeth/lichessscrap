//! The entire information a Lichess game and its moves provide. They can be cleared up for reusability purposes.

pub use game::Game;
pub use r#move::Move;

use log::{error, info, warn};
use pgn_reader::{Nag, SanPlus};
use shakmaty::{Outcome, Position};

use super::{
    attributes::{
        BoardConfiguration, Clk, Eco, Elo, Eval, Result as ResultAttr, Termination, TimeControl,
        Title, UTCDate, UTCTime, attribute::StringAttribute, move_descriptor::MoveDescriptor,
    },
    constants::{
        comments::{CLK, EVAL},
        headers::{
            BLACK, BLACK_ELO, BLACK_RATING_DIFF, BLACK_TITLE, DATE, ECO, EVENT, LICHESS_ID,
            OPENING, RESULT, ROUND, SITE, TERMINATION, TIME_CONTROL, UTC_DATE, UTC_TIME, WHITE,
            WHITE_ELO, WHITE_RATING_DIFF, WHITE_TITLE,
        },
    },
};
use crate::{loneerror, valuederror};

pub mod game;
pub mod r#move;

/// All the data in a PGN file as it is being read.
#[derive(Debug)]
pub struct Data {
    /// The number of games read.
    pub games: usize,
    /// The current game being analyzed.
    pub game: Game,
    /// The current move being analyzed.
    pub r#move: Move,
    /// The total moves of the current game being analyzed.
    pub moves: Vec<Move>,
    /// Whether processing this data led to any errors.
    pub has_errors: bool,
}

impl Default for Data {
    fn default() -> Self {
        info!("Constructing data for insertion.");
        Self {
            games: 0,
            game: Game::default(),
            r#move: Move::default(),
            moves: Vec::default(),
            has_errors: false,
        }
    }
}

impl Data {
    /// When a new game happens, game, move and moves must be reset.
    pub fn new_game(&mut self) {
        self.game.reset();
        self.r#move.reset();
        self.moves.clear();
        self.games += 1;
    }

    /// When a new move happens, move is advanced and parsed to board.
    pub fn new_move(&mut self, san: SanPlus) {
        if self.is_move_processed() {
            self.moves.push(self.r#move.clone());
        }
        self.r#move.next();

        match MoveDescriptor::from_and_play_san(&san, &mut self.game.chess) {
            Ok(value) => self.r#move.descriptor = value,
            Err(_) => {
                error!(
                    "{}.{} - Invalid SAN move played: {san}",
                    self.games, self.r#move.num
                );
                self.has_errors = true;
            }
        }
    }

    /// Processes the comment of a move, parsing the values of its fields.
    pub fn process_comment(&mut self, key: &[u8], value: &[u8]) {
        match key {
            CLK => match Clk::try_from(value) {
                Ok(value) => self.r#move.clk = Some(value),
                Err(e) => {
                    valuederror!(self, e);
                }
            },
            EVAL => match Eval::try_from(value) {
                Ok(value) => self.r#move.eval = Some(value),
                Err(e) => {
                    valuederror!(self, e);
                }
            },
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

    /// Processes the header of a game, parsing the values of its fields.
    pub fn process_header(&mut self, key: &[u8], value: &[u8]) {
        match key {
            SITE => (),
            LICHESS_ID => (),
            TIME_CONTROL => match TimeControl::try_from(value) {
                Ok(value) => self.game.time_control = value,
                Err(e) => {
                    valuederror!(self, e);
                }
            },
            RESULT => match ResultAttr::try_from(value) {
                Ok(value) => self.game.result = value,
                Err(e) => {
                    valuederror!(self, e);
                }
            },
            TERMINATION => match Termination::try_from(value) {
                Ok(value) => self.game.termination = value,
                Err(e) => {
                    valuederror!(self, e);
                }
            },
            DATE => (),
            UTC_DATE => match UTCDate::try_from(value) {
                Ok(value) => self.game.utc_date = value,
                Err(e) => {
                    valuederror!(self, e);
                }
            },
            UTC_TIME => match UTCTime::try_from(value) {
                Ok(value) => self.game.utc_time = value,
                Err(e) => {
                    valuederror!(self, e);
                }
            },
            OPENING => {
                if let Err(e) = self.game.opening.fill_ascii(value) {
                    valuederror!(self, e);
                }
            }
            ECO => match Eco::try_from(value) {
                Ok(value) => self.game.eco = value,
                Err(e) => {
                    valuederror!(self, e);
                }
            },
            EVENT => {
                if let Err(e) = self.game.ruleset.fill_ascii(value) {
                    valuederror!(self, e);
                }
            }
            ROUND => (),
            BLACK => {
                if let Err(e) = self.game.black.fill_ascii(value) {
                    valuederror!(self, e);
                }
            }
            BLACK_ELO => match Elo::try_from(value) {
                Ok(value) => self.game.black_elo = value,
                Err(e) => {
                    valuederror!(self, e);
                }
            },
            BLACK_RATING_DIFF => (),
            BLACK_TITLE => match Title::try_from(value) {
                Ok(value) => self.game.black_title = Some(value),
                Err(e) => {
                    valuederror!(self, e);
                }
            },
            WHITE => {
                if let Err(e) = self.game.white.fill_ascii(value) {
                    valuederror!(self, e);
                }
            }
            WHITE_ELO => match Elo::try_from(value) {
                Ok(value) => self.game.white_elo = value,
                Err(e) => {
                    valuederror!(self, e);
                }
            },
            WHITE_RATING_DIFF => (),
            WHITE_TITLE => match Title::try_from(value) {
                Ok(value) => self.game.white_title = Some(value),
                Err(e) => {
                    valuederror!(self, e);
                }
            },
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

    /// Adds the nag to the move's current value.
    pub fn add_nag(&mut self, nag: Nag) {
        self.r#move.descriptor.nag = nag;
    }

    /// Checks whether the outcome coincides with the outcome read.
    pub fn check_outcome(&mut self, outcome: Option<Outcome>) {
        if self.game.result != ResultAttr::from(outcome) {
            loneerror!("The outcome is different to the given result", self);
        }
    }

    /// Properly ends and processes the data of the game, setting the board configuration field and pushing the last move to moves.
    pub fn end_game(&mut self) {
        match BoardConfiguration::from_board(self.game.chess.board()) {
            Ok(value) => self.game.final_conf = value,
            Err((value, e)) => {
                self.game.final_conf = value;
                warn!("{} - {}", self.games, e);
            }
        };
        if self.is_move_processed() {
            self.moves.push(self.r#move.clone())
        }
    }

    /// Checks whether the current move is a fully processed move or not.
    pub const fn is_move_processed(&self) -> bool {
        self.r#move.num != 0
    }
}
