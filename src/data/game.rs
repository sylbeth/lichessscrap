//! The entire information a Lichess game provides. It can be cleared up for reusability purposes.

use shakmaty::Chess;

use crate::attributes::{
    //Date,
    Eco,
    Elo,
    FinalConfiguration,
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

    /// The chess position of this game.
    pub chess: Chess,

    /// The pieces left at the end of the game.
    pub final_conf: FinalConfiguration,
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

        self.chess = Chess::default();

        self.final_conf = FinalConfiguration::default();
    }
}
