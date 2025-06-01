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

/*
/// Finds the number of games in the given file.
fn find_games_aux<R: Read>(reader: R) -> Result<usize, Box<dyn Error>> {
    trace!("find_games_aux function.");
    debug!("{}", type_name_of_val(&reader));
    let mut count = 0;
    let mut n;
    let mut reader = BufReader::new(reader);

    loop {
        n = reader.skip_until(b'\n')?;
        if n == 0 {
            return Ok(count >> 1);
        } else if n == 1 {
            count += 1;
            if count & 0x7FFFFF == 0 {
                info!("Scanned {} games.", count >> 1);
            }
        }
    }
}
*/

fn find_games_aux<R: Read>(reader: R) -> Result<usize, Box<dyn Error>> {
    trace!("find_games_aux function.");
    debug!("{}", type_name_of_val(&reader));

    let mut count = 0;
    let mut prev = 0;
    let mut len = 0;
    let mut reader = BufReader::with_capacity(1 << 15, reader);
    let mut buff;

    loop {
        buff = reader.fill_buf()?;
        if prev + 1 == len {
            if let Some(first) = buff.first() {
                if *first == b'\n' {
                    count += 1;
                    if count & 0x7FFFFF == 0 {
                        info!("Scanned {} games.", count >> 1);
                    }
                }
            }
        }
        len = buff.len();
        prev = len;
        for double in memchr::memchr_iter(b'\n', buff) {
            if prev + 1 == double {
                count += 1;
                if count & 0x7FFFFF == 0 {
                    info!("Scanned {} games.", count >> 1);
                }
            }
            prev = double
        }
        if prev == len {
            return Ok(count >> 1);
        } else {
            reader.consume(len);
        }
    }
}

/*
/// Finds the number of games in the given file.
fn find_games_aux<R: Read>(reader: R) -> Result<usize, Box<dyn Error>> {
    trace!("find_games_aux function.");
    debug!("{}", type_name_of_val(&reader));

    let mut count = 0;
    let mut current;
    let mut len;
    let mut last_char_jump = false;
    let mut reader = BufReader::with_capacity(1 << 15, reader);
    let mut buff;

    loop {
        current = 0;
        buff = reader.fill_buf()?;
        len = buff.len();
        if last_char_jump {
            if let Some(first) = buff.first() {
                if *first == b'\n' {
                    count += 1;
                    if count & 0x7FFFFF == 0 {
                        info!("Scanned {} games.", count >> 1);
                    }
                }
            }
        }
        last_char_jump = false;
        for double in memchr::memmem::find_iter(buff, b"\n\n") {
            count += 1;
            if count & 0x7FFFFF == 0 {
                info!("Scanned {} games.", count >> 1);
            }
            current = double
        }
        if current == 0 {
            return Ok(count >> 1);
        } else {
            if buff[len - 1] == b'\n' {
                last_char_jump = true;
            }
            reader.consume(len);
        }
    }
}
*/

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
