use serde::{Deserialize, Serialize};

use crate::headers::*;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Game {
    pub id: usize,
    pub eco: Option<String>,
    pub event: Option<String>,
    pub opening: Option<String>,
    pub result: Option<String>,
    pub site: Option<String>,
    pub termination: Option<String>,
    pub time_control: Option<String>,
    pub utc_date: Option<String>,
    pub utc_time: Option<String>,
    pub white: Option<String>,
    pub white_elo: Option<String>,
    pub white_rating_diff: Option<String>,
    pub white_title: Option<String>,
    pub black: Option<String>,
    pub black_elo: Option<String>,
    pub black_rating_diff: Option<String>,
    pub black_title: Option<String>,
}

impl Game {
    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        let value = Some(match String::from_utf8(value.to_owned()) {
            Ok(str) => str,
            Err(_) => {
                let str = String::from_utf8_lossy(value);
                println!("Invalid UTF-8: {} <- {:?}", str, value);
                str.into_owned()
            }
        });
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
