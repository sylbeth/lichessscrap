use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Game {
    id: usize,
    eco: Option<String>,
    event: Option<String>,
    opening: Option<String>,
    result: Option<String>,
    site: Option<String>,
    termination: Option<String>,
    time_control: Option<String>,
    utc_date: Option<String>,
    utc_time: Option<String>,
    white: Option<String>,
    white_elo: Option<String>,
    white_rating_diff: Option<String>,
    white_title: Option<String>,
    black: Option<String>,
    black_elo: Option<String>,
    black_rating_diff: Option<String>,
    black_title: Option<String>,
}
