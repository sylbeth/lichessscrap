//! CLI argument parser for the scrapper.

use std::{error::Error, path::PathBuf};

use ::log::{info, trace};
use argfile::{PREFIX as FROMFILE_PREFIX, expand_args, parse_fromfile};
use clap::{Args, Parser};
use clap_verbosity_flag::{InfoLevel, Verbosity};

mod log;

/// CLI arguments to the Lichess scrapper, processed using Clap's derive API.
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct CLIArgs {
    /// PGN or ZSTD compressed PGN file to read data from.
    pub pgn_file: PathBuf,
    /// Whether the terminal should remain silent or not.
    #[arg(short, long)]
    pub silent: bool,
    /// File to log runtime information to.
    #[arg(short = 'l', long)]
    pub log_file: Option<PathBuf>,
    /// Clap verbosity flag.
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
    /// Arguments for checking data consistency.
    #[command(flatten)]
    pub consistency: ConsistencyArgs,
    /// Arguments for interacting with the mysql database.
    #[command(flatten)]
    pub database: DatabaseArgs,
}

impl CLIArgs {
    /// Parses the command line arguments alongside the argfile and initializes the loggers.
    ///
    /// # Errors
    /// Will return a [`Box`]ed [`Error`] if the parsing of arguments failed or the logging failed to initialize.
    pub fn parse_all() -> Result<Self, Box<dyn Error>> {
        let args = Self::parse_from(expand_args(parse_fromfile, FROMFILE_PREFIX)?);
        args.init_loggers()?;
        trace!("CLIArgs new function.");
        info!("Command line arguments read.");
        Ok(args)
    }
}

/// Subset of the CLI arguments used when checking for data consistency.
#[derive(Args, Debug)]
#[group(id = "consistency", multiple = true, requires = "check")]
pub struct ConsistencyArgs {
    /// Whether or not to check for data consistency.
    #[arg(short, long)]
    pub check: bool,
    /// Whether or not to print data found while checking.
    #[arg(short, long, conflicts_with = "sample")]
    pub print_collect: bool,
    /// File to write the data found while checking.
    #[arg(short, long, conflicts_with = "sample")]
    pub write_collect: Option<PathBuf>,
}

/// Subset of the CLI arguments used when connecting to the database.
#[derive(Args, Debug)]
#[group(id = "database", multiple = true)]
pub struct DatabaseArgs {
    /// Password of the database.
    #[arg(short, long = "envfile")]
    pub db_envfile: Option<PathBuf>,
    /// Total number of games in this file.
    #[arg(short = 'n', long, requires = "sample")]
    pub games: Option<usize>,
    /// To use a random sample of the file instead of using it all.
    #[arg(short = 'm', long = "sample-size")]
    pub sample: Option<usize>,
    /// Whether to rebuild the database or only build it if it doesn't exist.
    #[arg(short, long)]
    pub rebuild: bool,
    /// Whether or not to continue even with errors.
    #[arg(short, long, requires = "check")]
    pub force_insert: bool,
}
