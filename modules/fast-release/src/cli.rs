// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Maybe use a custom CLI in the future

use clap::{Arg, ArgAction, ArgMatches, Command};
use tracing::warn;

#[derive(Debug)]
pub struct CliParams {
  pub dry_run: bool,
  pub config_file_path: Option<String>,
}

pub fn get() -> CliParams {
  let clap: ArgMatches = Command::new("FastRelease CLI") /* .about("") */
    .arg(
      Arg::new("dry_run")
        .long("dry")
        .short('d')
        .action(ArgAction::SetTrue)
        .help("Disables release publishing"),
    )
    .arg(
      Arg::new("config_file_path")
        .long("config")
        .short('c')
        .help("Sets the path to a custom configuration file"),
    )
    .get_matches();

  let config_file_path: Option<String> = {
    let value: Option<String> = clap
      .get_one::<String>("config_file_path")
      .map(|v| v.to_string());
    if value.is_some() {
      warn!(
        message = "Using a custom configuration file.",
        category = "CLI"
      )
    }
    value
  };

  let dry_run: bool = {
    let value: bool = clap.get_one::<bool>("dry_run").unwrap_or(&false).to_owned();
    if value {
      warn!(message = "Forcing a dry run.", category = "CLI")
    }
    value
  };

  CliParams {
    config_file_path,
    dry_run,
  }
}
