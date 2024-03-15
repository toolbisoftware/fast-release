// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Maybe create a config file if the config file doesn't exist or it's empty
// Could be an argument on the CLI too

mod cli;
mod config;
mod constants;
mod error;

use crate::{cli::CliParams, config::Config};
use commonlib::{logger::LoggerBuilder, Logger};
use error::{soft_panic, FastReleaseError, FastReleaseErrorBuilder};
use tracing::info;

fn init_logger() -> Result<(), FastReleaseError> {
  match Logger::init(LoggerBuilder {
    level: Some(tracing::Level::DEBUG),
    file_logging: None,
  }) {
    Ok(_) => Ok(()),
    Err(error) => Err(
      FastReleaseErrorBuilder::new("Failed to initialize the logger.")
        .error(error)
        .get(),
    ),
  }
}

fn run() -> Result<(), FastReleaseError> {
  init_logger()?;
  info!("Running FastRelease v{}.", env!("CARGO_PKG_VERSION"));

  let cli_params: CliParams = cli::get();
  let config: Config = config::get(&cli_params)?;

  Ok(())
}

fn main() {
  match run() {
    Ok(_) => {}
    Err(error) => soft_panic(error),
  }
}
