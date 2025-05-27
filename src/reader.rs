//! Reader for the PGN files for this scrapper to scrap from.

use std::{
    error::Error,
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use log::trace;
use pgn_reader::{BufferedReader, Visitor};

#[cfg(feature = "zstd")]
use zstd::Decoder;

/// The buffered reader used for a PGN or ZSTD compressed PGN file.
pub enum PGNReader {
    /// A buffered reader for a simple PGN file.
    PGN(BufferedReader<File>),
    /// A buffered reader for a ZSTD compressed PGN file.
    #[cfg(feature = "zstd")]
    ZST(BufferedReader<Decoder<'static, BufReader<File>>>),
}

impl PGNReader {
    /// Creates a new buffered reader from the path to the file provided.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        trace!("PGNReader new function.");
        if let Some(ext) = path.as_ref().extension() {
            if ext == "zst" {
                #[cfg(feature = "zstd")]
                {
                    Ok(Self::ZST(BufferedReader::new(Decoder::new(File::open(
                        path,
                    )?)?)))
                }
                #[cfg(not(feature = "zstd"))]
                Err(
                    "The feature zstd must be active to be able to read a zstd file"
                        .to_owned()
                        .into(),
                )
            } else if ext == "pgn" {
                Ok(Self::PGN(BufferedReader::new(File::open(path)?)))
            } else {
                Err("Only zstd".to_owned().into())
            }
        } else {
            Err("Only zstd".to_owned().into())
        }
    }

    /// Reads a game from a PGN reader.
    #[allow(dead_code)]
    pub fn read_game<V: Visitor>(&mut self, visitor: &mut V) -> io::Result<Option<V::Result>> {
        match self {
            Self::PGN(file) => file.read_game(visitor),
            Self::ZST(file) => file.read_game(visitor),
        }
    }

    /// Reads all the games from a PGN reader.
    pub fn read_all<V: Visitor>(&mut self, visitor: &mut V) -> io::Result<()> {
        trace!("PGNReader read_all function.");
        match self {
            Self::PGN(file) => file.read_all(visitor),
            Self::ZST(file) => file.read_all(visitor),
        }
    }
}
