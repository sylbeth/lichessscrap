use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Move {
    game_id: usize,
    num: usize,
    san: String,
    nag: Option<u8>,
    clk: Option<String>,
    eval: Option<String>,
}
