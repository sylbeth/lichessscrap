use serde::{Deserialize, Serialize};

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
