use std::{env::args, error::Error, fs::File};

use pgn_reader::BufferedReader;

use crawler::Crawler;
use game::Game;
use r#move::Move;

mod collector;
mod comments;
mod crawler;
mod game;
mod headers;
mod r#move;
mod serializer;
mod stats;

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(file) = args().nth(1) {
        let mut pgn = BufferedReader::new(zstd::Decoder::new(File::open(file)?)?);
    
        let mut crawler = Crawler::new()?;
        pgn.read_all(&mut crawler)?;
    
        crawler.collector.print_comments();
        println!();
    
        crawler.collector.print_headers();
        println!();
    
        print!("{}", crawler.stats);
    }

    Ok(())
}
