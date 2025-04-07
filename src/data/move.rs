use std::str::from_utf8;

use crate::constants::comments::*;

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "PascalCase")
)]
pub struct Move {
    pub game_id: usize,
    pub num: usize,
    pub san: String,
    pub nag: Option<u8>,
    pub eval: String,
    pub clk: String,
}

impl Move {
    pub fn reset(&mut self) {
        self.san.clear();
        self.nag = None;
        self.clk.clear();
        self.eval.clear();
    }

    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        let value = match from_utf8(value) {
            Ok(str) => str,
            Err(_) => {
                let str = String::from_utf8_lossy(value);
                panic!("Invalid UTF-8: {} <- {:?}", str, value);
            }
        };
        match key {
            CLK => self.clk.push_str(value),
            EVAL => self.eval.push_str(value),
            key => println!(
                "New comment found: {} <- {:?}",
                String::from_utf8_lossy(key),
                key
            ),
        }
    }
}

