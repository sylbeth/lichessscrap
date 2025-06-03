//! Main module of the scrapper, which handles the parsing of the CLI arguments, opens the data file and processes it.

use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufWriter, stdout},
};

use dotenvy::{dotenv, from_filename as dotenv_from_filename};
use log::{debug, info, trace, warn};

use args::CLIArgs;
use pgn_reader::BufferedReader;
use reader::PGNReader;
use visitors::checkcollect::{CheckerCollector, checker::Checker};

use crate::{
    reader::{PGNSampler, find_games},
    visitors::database::Database,
};

mod adapter;
mod args;
mod reader;
mod visitors;

/// Crawls the file and analyzes it completely.
fn full_crawling(args: CLIArgs) -> Result<(), Box<dyn Error>> {
    trace!("full_crawling function.");
    info!("Commencing crawling of the file wile analyzing it fully.");

    let mut checking_errors = false;

    if args.consistency.check {
        info!("Checking the PGN file for data.");
        let mut pgn = PGNReader::new(&args.pgn_file)?;
        let checker = if args.consistency.print_collect | args.consistency.write_collect.is_some() {
            info!("Starting both checking and collecting the PGN file.");
            let mut checker_collector = CheckerCollector::default();
            pgn.read_all(&mut checker_collector)?;
            info!("Checking and collection finished.");
            debug!("{checker_collector:?}");
            if args.consistency.print_collect {
                info!("Printing the collection to stdout.");
                checker_collector.write_collection(&mut stdout())?;
            }
            if let Some(collect_file) = args.consistency.write_collect {
                info!("Writing the collection to {collect_file:?}.");
                checker_collector
                    .write_collection(&mut BufWriter::new(File::create(collect_file)?))?;
            }
            checker_collector.checker
        } else {
            info!("Starting only checking the PGN file.");
            let mut checker = Checker::default();
            pgn.read_all(&mut checker)?;
            info!("Checking finished.");
            debug!("{checker:?}");
            checker
        };
        checking_errors = checker.has_errors;
        if checker.has_errors {
            warn!("The checking finished with errors.")
        } else {
            info!("The checking finished without errors.")
        }
    }

    if checking_errors & !args.database.force_insert {
        info!("Since there were errors during the check, the program will stop.");
        info!(
            "If this is not the intended behaviour, consider using the flag \"-f\"/\"--force-insert\"."
        );
        return Err("There were errors during the check process.".into());
    }

    if let Ok(db_url) = env::var("DATABASE_URL") {
        info!("Inserting the full PGN file's data into the database.");
        let mut pgn = PGNReader::new(&args.pgn_file)?;
        let mut database =
            Database::new(&db_url, args.database.rebuild, args.database.max_threads)?;
        pgn.read_all(&mut database)?;
        if database.data.has_errors {
            warn!("The database insertion finished with parsing errors.");
        } else {
            info!("The database insertion finished without errors.");
        }
        database.finish_insertion();
    }

    info!("Full crawling finished.");

    Ok(())
}

/// Crawls the file while only analyzing a given sample.
fn sample_crawling(args: CLIArgs, sample: usize) -> Result<(), Box<dyn Error>> {
    trace!("sample_crawling function.");
    info!("Commencing crawling of the file wile analyzing only a sample.");

    let db_url = env::var("DATABASE_URL").map_err(|_| io::Error::new(io::ErrorKind::NotFound, "The database url was not found. Please provide the environment variable DATABASE_URL as mysql://user:password@localhost, for example, through a .env file."))?;
    let games = if let Some(games) = args.database.games {
        games
    } else {
        find_games(&args.pgn_file)?
    };

    info!("The file contains {games} games.");

    if let Some(ext) = args.pgn_file.extension() {
        if ext == "zst" {
            #[cfg(feature = "zstd")]
            {
                use zstd::Decoder;

                let mut sampler = PGNSampler::new(
                    Decoder::new(File::open(&args.pgn_file)?)?,
                    sample,
                    games,
                    &args.pgn_file,
                );
                info!("Starting the insertion of the sample.");
                let mut database =
                    Database::new(&db_url, args.database.rebuild, args.database.max_threads)?;
                let mut cursor;
                if args.consistency.check {
                    let mut checker = Checker::default();
                    while sampler.fill_next_game()? {
                        cursor = BufferedReader::new_cursor(&sampler.current_data);
                        cursor.read_game(&mut checker)?;
                        if checker.has_errors & !args.database.force_insert {
                            info!(
                                "Since there were errors during the check, the program will stop."
                            );
                            info!(
                                "If this is not the intended behaviour, consider using the flag \"-f\"/\"--force-insert\"."
                            );
                            return Err("There were errors during the check process.".into());
                        }
                        cursor = BufferedReader::new_cursor(&sampler.current_data);
                        cursor.read_game(&mut database)?;
                    }
                } else {
                    while sampler.fill_next_game()? {
                        cursor = BufferedReader::new_cursor(&sampler.current_data);
                        cursor.read_game(&mut database)?;
                    }
                }
                database.finish_insertion();
                info!("Finished processing the sample.");
                Ok(())
            }
            #[cfg(not(feature = "zstd"))]
            Err(Box::new(io::Error::new(
                io::ErrorKind::Unsupported,
                "The feature zstd must be active to be able to read a zstd file",
            )))
        } else if ext == "pgn" {
            let mut sampler =
                PGNSampler::new(File::open(&args.pgn_file)?, sample, games, &args.pgn_file);
            info!("Starting the insertion of the sample.");
            let mut database =
                Database::new(&db_url, args.database.rebuild, args.database.max_threads)?;
            let mut cursor;
            if args.consistency.check {
                let mut checker = Checker::default();
                while sampler.fill_next_game()? {
                    cursor = BufferedReader::new_cursor(&sampler.current_data);
                    cursor.read_game(&mut checker)?;
                    if checker.has_errors & !args.database.force_insert {
                        info!("Since there were errors during the check, the program will stop.");
                        info!(
                            "If this is not the intended behaviour, consider using the flag \"-f\"/\"--force-insert\"."
                        );
                        return Err("There were errors during the check process.".into());
                    }
                    cursor = BufferedReader::new_cursor(&sampler.current_data);
                    cursor.read_game(&mut database)?;
                }
            } else {
                while sampler.fill_next_game()? {
                    cursor = BufferedReader::new_cursor(&sampler.current_data);
                    cursor.read_game(&mut database)?;
                }
            }
            database.finish_insertion();
            info!("Finished processing the sample.");
            Ok(())
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Only zstd and pgn files are allowed",
            )))
        }
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Only zstd and pgn files are allowed",
        )))
    }
}

/// Main function of the scrapper.
fn main() -> Result<(), Box<dyn Error>> {
    let args = CLIArgs::parse_all()?;
    #[cfg(feature = "full-collect")]
    info!("Feature full-collect is active.");
    #[cfg(feature = "full-check")]
    info!("Feature full-check is active.");
    #[cfg(feature = "zstd")]
    info!("Feature zstd is active.");
    #[cfg(feature = "chrono")]
    info!("Feature chrono is active.");
    #[cfg(feature = "time")]
    info!("Feature time is active.");
    #[cfg(feature = "csv")]
    info!("Feature csv is active.");
    #[cfg(feature = "chrono-serde")]
    info!("Feature chrono-serde is active.");
    #[cfg(feature = "time-serde")]
    info!("Feature time-serde is active.");
    #[cfg(feature = "chrono-mysql")]
    info!("Feature chrono-mysql is active.");
    #[cfg(feature = "time-mysql")]
    info!("Feature time-mysql is active.");
    #[cfg(feature = "chrono-diesel")]
    info!("Feature chrono-diesel is active.");
    #[cfg(feature = "time-diesel")]
    info!("Feature time-diesel is active.");
    trace!("main function.");
    debug!("{args:?}");

    if let Some(envfile) = &args.database.db_envfile {
        info!("Reading dotenv from given file.");
        dotenv_from_filename(envfile)?;
        info!("dotenv file read.");
    } else {
        info!("Reading dotenv.");
        if dotenv().is_ok() {
            info!("dotenv file read.");
        } else {
            info!("dotenv file not found, proceeding with checking.");
        }
    }

    if let Some(sample) = args.database.sample {
        sample_crawling(args, sample)
    } else {
        full_crawling(args)
    }
}
