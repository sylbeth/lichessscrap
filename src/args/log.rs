//! Configuration of this scrapper's logging.

use std::{error::Error, fs::File, io::BufWriter};

use log::{info, trace, Level};
use simplelog::{
    Color, ColorChoice::Auto as AutoColor, CombinedLogger, ConfigBuilder, TermLogger,
    TerminalMode::Mixed as MixedTerm, WriteLogger,
};
use time::macros::format_description;

use super::CLIArgs;

impl CLIArgs {
    /// Configures and initializes the loggers for the scrapper.
    pub fn init_loggers(&self) -> Result<(), Box<dyn Error>> {
        if self.silent & self.log_file.is_none() {
            return Ok(());
        }
        let level_filter = self.verbose.log_level_filter();

        let config = ConfigBuilder::new()
            .set_level_color(Level::Trace, Some(Color::Yellow))
            .set_level_color(Level::Debug, Some(Color::Green))
            .set_level_color(Level::Info, Some(Color::Cyan))
            .set_level_color(Level::Warn, Some(Color::Magenta))
            .set_level_color(Level::Error, Some(Color::Red))
            .set_time_format_custom(format_description!(
                "[[[hour]:[minute]:[second].[subsecond digits:3]]"
            ))
            .build();
        match (self.silent, &self.log_file) {
            (false, Some(log_file)) => CombinedLogger::init(vec![
                TermLogger::new(level_filter, config.clone(), MixedTerm, AutoColor),
                WriteLogger::new(
                    level_filter,
                    config,
                    BufWriter::new(File::create(log_file)?),
                ),
            ])?,
            (false, None) => TermLogger::init(level_filter, config, MixedTerm, AutoColor)?,
            (true, Some(log_file)) => WriteLogger::init(
                level_filter,
                config,
                BufWriter::new(File::create(log_file)?),
            )?,
            _ => (),
        }
        trace!("CLIArgs init_loggers function.");
        info!("Logging initialized.");

        Ok(())
    }
}
