use crate::headers::*;

#[derive(Debug, Default)]
pub struct Game {
    pub id: usize,
    pub eco: String,
    pub event: String,
    pub opening: String,
    pub result: String,
    pub site: String,
    pub termination: String,
    pub time_control: String,
    pub utc_date: String,
    pub utc_time: String,
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
        self.eco.clear();
        self.event.clear();
        self.opening.clear();
        self.result.clear();
        self.site.clear();
        self.termination.clear();
        self.time_control.clear();
        self.utc_date.clear();
        self.utc_time.clear();
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
            BLACK => self.black = value,
            BLACK_ELO => self.black_elo = value,
            BLACK_RATING_DIFF => self.black_rating_diff = value,
            BLACK_TITLE => self.black_title = value,
            ECO => self.eco = value,
            EVENT => self.event = value,
            OPENING => self.opening = value,
            RESULT => self.result = value,
            SITE => self.site = value,
            TERMINATION => self.termination = value,
            TIME_CONTROL => self.time_control = value,
            UTC_DATE => self.utc_date = value,
            UTC_TIME => self.utc_time = value,
            WHITE => self.white = value,
            WHITE_ELO => self.white_elo = value,
            WHITE_RATING_DIFF => self.white_rating_diff = value,
            WHITE_TITLE => self.white_title = value,
            key => println!(
                "New header found: {} <- {:?}",
                String::from_utf8_lossy(key),
                key
            ),
        }
    }
}
