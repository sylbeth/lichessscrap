//! The entire information a Lichess game provides. It can be cleared up for reusability purposes.

use mysql::{Params, params};
use shakmaty::Chess;

use crate::attributes::{
    //Date,
    Eco,
    Elo,
    BoardConfiguration,
    Opening,
    Player,
    Result,
    RuleSet,
    Termination,
    TimeControl,
    Title,
    UTCDate,
    UTCTime,
    attribute::StringAttribute,
    datetime::Datetime,
};

/// Struct containing all the information of a Lichess game.
#[derive(Debug, Default, Clone)]
pub struct Game {
    /*
    /// The website this game was played at.
    pub site: String,
    */
    /// The time control this game used.
    pub time_control: TimeControl,
    /// The result of this game.
    pub result: Result,
    /// The termination of this game.
    pub termination: Termination,
    /*
    /// The date this game was played at.
    pub date: Date,
    */
    /// The UTC date this game was played at.
    pub utc_date: UTCDate,
    /// The UTC time this game was played at.
    pub utc_time: UTCTime,

    /// The opening that was played in this game.
    pub opening: Opening,
    /// The ECO code of the opening.
    pub eco: Eco,

    /// The ruleset this game was played under.
    pub ruleset: RuleSet,

    /*
    /// The round of the game.
    pub round: (),
    */
    /// The username of the black player.
    pub black: Player,
    /// The elo of the black player.
    pub black_elo: Elo,
    /*
    /// The rating difference of the black player.
    pub black_rating_diff: Option<i16>,
    */
    /// The title of the black player.
    pub black_title: Option<Title>,

    /// The username of the white player.
    pub white: Player,
    /// The elo of the white player.
    pub white_elo: Elo,
    /*
    /// The rating difference of the white player.
    pub white_rating_diff: Option<i16>,
    */
    /// The title of the white player.
    pub white_title: Option<Title>,

    /// Whether or not this game has a clock.
    pub has_clock: bool,
    /// Whether or not this game has stockfish evaluations.
    pub has_evaluations: bool,

    /// The chess position of this game.
    pub chess: Chess,

    /// The board configuration at the end of the game.
    pub final_conf: BoardConfiguration,
}

impl Game {
    /// Resets the [`Game`] to its original state without deallocating.
    pub fn reset(&mut self) {
        //self.site.clear();
        self.time_control = TimeControl(None);
        self.result = Result::Null;
        self.termination = Termination::Unterminated;

        //self.date = Date::default();
        self.utc_date = UTCDate::default();
        self.utc_time = UTCTime::default();

        self.opening.clear();
        self.eco = Eco::default();

        self.ruleset.reset();
        //self.round;

        self.black.clear();
        self.black_elo.0 = None;
        //self.black_rating_diff = None;
        self.black_title = None;

        self.white.clear();
        self.white_elo.0 = None;
        //self.white_rating_diff = None;
        self.white_title = None;

        self.has_clock = false;
        self.has_evaluations = false;

        self.chess = Chess::default();

        self.final_conf = BoardConfiguration::default();
    }

    /// Prepares the parameters for MySQL insertion of this data.
    pub fn as_params(
        &self,
        ruleset_id: u64,
        opening_id: Option<u64>,
        fc_id: u64,
        white: Option<u64>,
        black: Option<u64>,
    ) -> Params {
        let (start_time, increment) = self.time_control.0.unzip();
        params! {
            ruleset_id,
            opening_id,
            fc_id,
            white,
            "white_elo" => self.white_elo.0,
            "white_title" => self.white_title,
            black,
            "black_elo" => self.black_elo.0,
            "black_title" => self.black_title,
            start_time,
            increment,
            "result" => self.result,
            "termination" => self.termination,
            "datetime" => Datetime::new(self.utc_date.0, self.utc_time.0).0, "has_clock" => self.has_clock,
            "has_evaluations" => self.has_evaluations,
        }
    }
}
