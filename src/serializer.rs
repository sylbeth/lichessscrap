use std::fs::File;

use csv::Writer;

#[derive(Debug)]
pub struct Serializer {
    pub games: Writer<File>,
    pub moves: Writer<File>,
}
