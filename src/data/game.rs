use crate::constants::headers::{
    BLACK, BLACK_ELO, BLACK_RATING_DIFF, BLACK_TITLE, DATE, ECO, EVENT, OPENING, RESULT, ROUND,
    SITE, TERMINATION, TIME_CONTROL, UTC_DATE, UTC_TIME, WHITE, WHITE_ELO, WHITE_RATING_DIFF,
    WHITE_TITLE,
};

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "PascalCase")
)]
pub struct Game {
    pub game_id: usize,

    pub site: String,
    pub time_control: String,
    pub result: String,
    pub termination: String,

    pub date: String,
    #[cfg_attr(feature = "serde", serde(rename = "UTCDate"))]
    pub utc_date: String,
    #[cfg_attr(feature = "serde", serde(rename = "UTCTime"))]
    pub utc_time: String,

    pub opening: String,
    #[cfg_attr(feature = "serde", serde(rename = "ECO"))]
    pub eco: String,

    pub event: String,
    pub round: String,

    pub white: String,
    pub white_elo: String,
    pub white_rating_diff: String,
    pub white_title: String,

    pub black: String,
    pub black_elo: String,
    pub black_rating_diff: String,
    pub black_title: String,
}

impl Game {
    pub fn reset(&mut self) {
        self.site.clear();
        self.time_control.clear();
        self.result.clear();
        self.termination.clear();

        self.date.clear();
        self.utc_date.clear();
        self.utc_time.clear();

        self.opening.clear();
        self.eco.clear();

        self.event.clear();
        self.round.clear();

        self.white.clear();
        self.white_elo.clear();
        self.white_rating_diff.clear();
        self.white_title.clear();

        self.black.clear();
        self.black_elo.clear();
        self.black_rating_diff.clear();
        self.black_title.clear();
    }

    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        let value = match String::from_utf8(value.to_owned()) {
            Ok(str) => str,
            Err(_) => {
                let str = String::from_utf8_lossy(value);
                println!("Invalid UTF-8: {} <- {:?}", str, value);
                str.into_owned()
            }
        };
        match key {
            SITE => self.site = value,
            TIME_CONTROL => self.time_control = value,
            RESULT => self.result = value,
            TERMINATION => self.termination = value,
            DATE => self.date = value,
            UTC_DATE => self.utc_date = value,
            UTC_TIME => self.utc_time = value,
            OPENING => self.opening = value,
            ECO => self.eco = value,
            EVENT => self.event = value,
            ROUND => self.round = value,
            WHITE => self.white = value,
            WHITE_ELO => self.white_elo = value,
            WHITE_RATING_DIFF => self.white_rating_diff = value,
            WHITE_TITLE => self.white_title = value,
            BLACK => self.black = value,
            BLACK_ELO => self.black_elo = value,
            BLACK_RATING_DIFF => self.black_rating_diff = value,
            BLACK_TITLE => self.black_title = value,
            key => println!(
                "New header found: {} <- {:?}",
                String::from_utf8_lossy(key),
                key
            ),
        }
    }
}

