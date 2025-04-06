use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Move {
    pub game_id: usize,
    pub num: usize,
    pub san: String,
    pub nag: Option<u8>,
    pub clk: Option<String>,
    pub eval: Option<String>,
}
