// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Maybe use a propietary system in the future.

use clap::{Arg, ArgAction, ArgMatches, Command};
use tracing::warn;

#[derive(Debug)]
pub struct CliParams {
  pub config_file_path: Option<String>,
  pub dry_run: bool,
}

pub fn get() -> CliParams {
  let mut cli_params: CliParams = CliParams {
    config_file_path: None,
    dry_run: false,
  };
  let clap: ArgMatches = Command::new("FastRelease CLI")
    // .about("")
    .arg(
      Arg::new("dry")
        .long("dry")
        .short('d')
        .action(ArgAction::SetTrue)
        .help("Disables publishing a new release for the current run"),
    )
    .arg(
      Arg::new("config")
        .long("config")
        .short('c')
        .help("Sets the path to a custom configuration file"),
    )
    .get_matches();

  if let Some(dry) = clap.get_one::<bool>("dry") {
    let dry = dry.to_owned();
    if dry {
      warn!(
        message = "Running in dry mode. A release won't be published.",
        category = "CLI"
      );
    }
    cli_params.dry_run = dry;
  }
  if let Some(config) = clap.get_one::<String>("config") {
    warn!(
      message = "Using a custom set configuration file.",
      category = "CLI"
    );
    cli_params.config_file_path = Some(config.to_owned())
  }

  cli_params
}
