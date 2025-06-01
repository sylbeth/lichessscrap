//! Reader for the PGN files for this scrapper to scrap from.

use std::{
    any::type_name_of_val,
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::{Path, PathBuf},
    vec::IntoIter,
};

use log::{debug, info, trace};
use pgn_reader::{BufferedReader, Visitor};
use rand::seq::index::sample;
use rand_seeder::{Seeder, SipRng};

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
    ///
    /// # Errors
    /// Will return [`io::Error`] if the file could not be opened.
    pub fn new<P: AsRef<Path> + Debug>(path: P) -> io::Result<Self> {
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
                Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "The feature zstd must be active to be able to read a zstd file",
                ))
            } else if ext == "pgn" {
                Ok(Self::PGN(BufferedReader::new(File::open(path)?)))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Only zstd and pgn files are allowed",
                ))
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Only zstd and pgn files are allowed",
            ))
        }
    }

    /// Reads a game from a PGN reader.
    ///
    /// # Errors
    /// Will return [`io::Error`] if the game could not be read.
    #[allow(dead_code)]
    pub fn read_game<V: Visitor>(&mut self, visitor: &mut V) -> io::Result<Option<V::Result>> {
        match self {
            Self::PGN(file) => file.read_game(visitor),
            Self::ZST(file) => file.read_game(visitor),
        }
    }

    /// Reads all the games from a PGN reader.
    ///
    /// # Errors
    /// Will return [`io::Error`] if the games could not be read.
    pub fn read_all<V: Visitor>(&mut self, visitor: &mut V) -> io::Result<()> {
        trace!("PGNReader read_all function.");
        match self {
            Self::PGN(file) => file.read_all(visitor),
            Self::ZST(file) => file.read_all(visitor),
        }
    }
}

/*
/// Finds the number of games in the given file reader.
fn find_games_aux<R: Read>(reader: R) -> io::Result<usize> {
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

/// Finds the number of games in the given file reader.
fn find_games_aux<R: Read>(reader: R) -> io::Result<usize> {
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
/// Finds the number of games in the given file reader.
fn find_games_aux<R: Read>(reader: R) -> io::Result<usize> {
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
pub fn find_games<P: AsRef<Path> + Debug>(path: P) -> io::Result<usize> {
    trace!("find_games function.");
    debug!("{path:?}");
    if let Some(ext) = path.as_ref().extension() {
        if ext == "zst" {
            #[cfg(feature = "zstd")]
            {
                find_games_aux(Decoder::new(File::open(path)?)?)
            }
            #[cfg(not(feature = "zstd"))]
            Err(Box::new(io::Error::new(
                io::ErrorKind::Unsupported,
                "The feature zstd must be active to be able to read a zstd file",
            )))
        } else if ext == "pgn" {
            find_games_aux(File::open(path)?)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Only zstd and pgn files are allowed",
            ))
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Only zstd and pgn files are allowed",
        ))
    }
}

/// A sampler for a PGN file. Takes a sample of usizes and retrieves the data for making cursors for each of the games.
pub struct PGNSampler<R: Read> {
    /// PGN file reader, with a buffer surrounding it.
    reader: BufReader<R>,
    /// Sample of indices, which all must be valid.
    sample: IntoIter<usize>,
    /// Counter for the scanned games.
    game_counter: usize,
    /// Counter for the processed games.
    processed_counter: usize,
    /// Divisor used when showing information of how many games have been processed.
    divisor: usize,
    /// The data currently available for use. Only valid after a call to `fill_next_game`.
    pub current_data: Vec<u8>,
}

impl<R: Read> PGNSampler<R> {
    /// Constructs a new PGN sampler, creating the sample and buffering the reader.
    pub fn new(reader: R, sample_size: usize, total_games: usize, seed: &PathBuf) -> Self {
        trace!("PGNSampler new function.");
        info!("Creating the sampler.");
        let mut indices = sample::<SipRng>(
            &mut Seeder::from(seed.file_name()).into_rng(),
            total_games,
            sample_size,
        )
        .into_vec();
        indices.sort();
        let divisor = sample_size >> 6;
        Self {
            reader: BufReader::with_capacity(1 << 15, reader),
            sample: indices.into_iter(),
            game_counter: 0,
            processed_counter: 0,
            divisor,
            current_data: Vec::new(),
        }
    }

    /// Fills the internal `current_data` with all the data for the immediate next game.
    ///
    /// # Errors
    /// Will return [`io::Error`] if the internal buffer could not be filled or if there is an error with the sampling.
    fn fill_game(&mut self) -> io::Result<()> {
        let mut buff;
        let mut prev;
        let mut len;
        self.current_data.clear();

        buff = self.reader.fill_buf()?;
        len = buff.len();
        prev = len;
        for double in memchr::memchr_iter(b'\n', buff) {
            if prev + 1 == double {
                self.game_counter += 1;

                if self.game_counter & 1 == 0 {
                    prev += 2;
                    self.current_data.extend_from_slice(&buff[0..prev]);
                    if self.game_counter & 0x7FFFFF == 0 {
                        info!("Iterated over {} games.", self.game_counter >> 1);
                    }
                    self.reader.consume(prev);
                    return Ok(());
                }
            }
            prev = double
        }

        self.current_data.extend_from_slice(buff);
        self.reader.consume(len);

        buff = self.reader.fill_buf()?;
        if prev + 1 == len {
            if let Some(first) = buff.first() {
                if *first == b'\n' {
                    self.game_counter += 1;

                    if self.game_counter & 1 == 0 {
                        self.current_data.push(b'\n');
                        if self.game_counter & 0x7FFFFF == 0 {
                            info!("Iterated over {} games.", self.game_counter >> 1);
                        }
                        self.reader.consume(1);
                        return Ok(());
                    }
                }
            }
        }
        len = buff.len();
        prev = len;

        for double in memchr::memchr_iter(b'\n', buff) {
            if prev + 1 == double {
                self.game_counter += 1;

                if self.game_counter & 1 == 0 {
                    prev += 2;
                    self.current_data.extend_from_slice(&buff[0..prev]);
                    if self.game_counter & 0x7FFFFF == 0 {
                        info!("Iterated over {} games.", self.game_counter >> 1);
                    }
                    self.reader.consume(prev);
                    return Ok(());
                }
            }
            prev = double
        }
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "A full game should fit in the buffer, so there is no way to end up here.",
        ))
    }

    /// Fills the internal `current_data` with all the data for the next game to sample, unless there aren't any. Returns whether there is new data or not.
    ///
    /// # Errors
    /// Will return [`io::Error`] if the internal buffer could not be filled or if there is an error with the sampling and the EOF is reached prematurely.
    pub fn fill_next_game(&mut self) -> io::Result<bool> {
        let current_game = if let Some(current_game) = self.sample.next() {
            self.processed_counter += 1;
            if self.processed_counter % self.divisor == 0 {
                info!(
                    "Processed {} games, currently processing {}.",
                    self.processed_counter - 1,
                    current_game
                );
            }
            // To check against the counter, which counts every two.
            current_game << 1
        } else {
            return Ok(false);
        };

        if self.game_counter == current_game {
            self.fill_game()?;
            return Ok(true);
        }

        let mut prev = 0;
        let mut len = 0;
        let mut buff;

        loop {
            buff = self.reader.fill_buf()?;
            if prev + 1 == len {
                if let Some(first) = buff.first() {
                    if *first == b'\n' {
                        self.game_counter += 1;
                        if self.game_counter & 0x7FFFFF == 0 {
                            info!("Iterated over {} games.", self.game_counter >> 1);
                        }
                        if self.game_counter == current_game {
                            self.reader.consume(1);
                            self.fill_game()?;
                            return Ok(true);
                        }
                    }
                }
            }
            len = buff.len();
            prev = len;
            for double in memchr::memchr_iter(b'\n', buff) {
                if prev + 1 == double {
                    self.game_counter += 1;
                    if self.game_counter & 0x7FFFFF == 0 {
                        info!("Iterated over {} games, processed.", self.game_counter >> 1);
                    }
                    if self.game_counter == current_game {
                        self.reader.consume(double + 1);
                        self.fill_game()?;
                        return Ok(true);
                    }
                }
                prev = double
            }
            if len == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "All the games to iterate over should exist, thus, there should be no EOF.",
                ));
            } else {
                self.reader.consume(len);
            }
        }
    }
}
