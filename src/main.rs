//! Main module of the scrapper, which handles the parsing of the CLI arguments, opens the data file and processes it.

use std::{
    error::Error,
    fs::File,
    io::{BufWriter, stdout},
};

use log::{debug, info, trace, warn};
use rand::{rng, seq::index::sample};

use args::CLIArgs;
use reader::PGNReader;
use visitors::{
    checkcollect::{CheckerCollector, checker::Checker},
    stats::Stats,
};

mod args;
mod reader;
mod visitors;

/// The part of the entire games that will be used for sampling. If there are `N` games, then the sample will consist of `N/GAMES_SAMPLE_DIVISOR` games.
const GAMES_SAMPLE_DIVISOR: usize = 1000;

/// Main function of the scrapper.
fn main() -> Result<(), Box<dyn Error>> {
    let args = CLIArgs::parse_all()?;
    #[cfg(feature = "full-collect")]
    info!("Feature full-collect is active.");
    #[cfg(feature = "full-check")]
    info!("Feature full-check is active.");
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
    #[cfg(feature = "memchr")]
    info!("Feature memchr is active.");
    #[cfg(feature = "zstd")]
    info!("Feature zstd is active.");
    trace!("main function.");
    debug!("{args:?}");

    let mut games = args.games;
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
        games = Some(checker.games);
        info!("Total games read: {}", checker.games);
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

    if let Some((db_password, op_mode)) = args.database.db_password.zip(args.database.op_mode) {
        info!(
            "Inserting the PGN file's data into the database with mode {:?}.",
            op_mode
        );
        let games = if let Some(games) = games {
            warn!(
                "Number of games provided, if incorrect would result to erroneous data sample collection."
            );
            games
        } else {
            info!("Number of games not provided, will proceed to read the file to count it.");
            let mut stats = Stats::default();
            let mut pgn = PGNReader::new(&args.pgn_file)?;
            pgn.read_all(&mut stats)?;
            info!("Counting finished.");
            debug!("{stats:?}");
            stats.log();
            stats.games
        };
        info!("Database insertion finished.");
    }

    info!("Program finished.");

    Ok(())
}
