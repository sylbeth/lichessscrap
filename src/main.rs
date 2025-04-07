use std::{env::args, error::Error, fs::File};

use pgn_reader::BufferedReader;

use visitors::crawler::Crawler;

mod args;
mod visitors;

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(file) = args().nth(1) {
        let mut pgn = BufferedReader::new(zstd::Decoder::new(File::open(file)?)?);

        let mut crawler = Crawler::new();
        pgn.read_all(&mut crawler)?;
        crawler.show();
    }

    Ok(())
}
