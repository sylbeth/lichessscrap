//! Reader for the PGN files for this scrapper to scrap from.

use std::{
    any::type_name_of_val,
    error::Error,
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

use log::{debug, info, trace};
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
    pub fn new<P: AsRef<Path> + Debug>(path: P) -> Result<Self, Box<dyn Error>> {
        trace!("PGNReader new function.");
        debug!("{path:?}");
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
                Err("Only zstd and pgn files are allowed".to_owned().into())
            }
        } else {
            Err("Only zstd and pgn files are allowed".to_owned().into())
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

/// Finds the number of games in the given file.
fn find_games_aux<R: Read>(reader: R) -> Result<usize, Box<dyn Error>> {
    trace!("find_games_aux function.");
    debug!("{}", type_name_of_val(&reader));
    let mut count = 0;
    let mut n;
    let mut reader = BufReader::new(reader);
    let mut line = Vec::new();

    loop {
        n = reader.read_until(b'\n', &mut line)?;
        if n == 0 {
            // Given that right shift truncates the last bit, we just want to ensure that if it's odd it is added 1 (there has been a formatting error) and if not it stays the same.
            return Ok((count + 1) >> 1);
        } else if (n == 1) & (line[0] == b'\n') {
            count += 1;
            if count % 1000000 == 0 {
                info!("Scanned {count} games.");
            }
        }
        line.clear();
    }
}

/// Finds the number of games in the given file.
pub fn find_games<P: AsRef<Path> + Debug>(path: P) -> Result<usize, Box<dyn Error>> {
    trace!("find_games function.");
    debug!("{path:?}");
    if let Some(ext) = path.as_ref().extension() {
        if ext == "zst" {
            #[cfg(feature = "zstd")]
            {
                find_games_aux(Decoder::new(File::open(path)?)?)
            }
            #[cfg(not(feature = "zstd"))]
            Err(
                "The feature zstd must be active to be able to read a zstd file"
                    .to_owned()
                    .into(),
            )
        } else if ext == "pgn" {
            find_games_aux(File::open(path)?)
        } else {
            Err("Only zstd and pgn files are allowed".to_owned().into())
        }
    } else {
        Err("Only zstd and pgn files are allowed".to_owned().into())
    }
}
