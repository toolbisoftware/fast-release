// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::io::Error;
use tracing::{error, warn};

#[derive(Debug)]
pub struct FastReleaseError<'a> {
  pub message: &'a str,
  pub error: Option<Error>,
}

impl std::fmt::Display for FastReleaseError<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let message: String = format!("{}", self.message);
    let error: String = if let Some(value) = &self.error {
      format!("\n{}", value)
    } else {
      "".into()
    };

    write!(f, "{}{}", message, error)
  }
}

pub fn throw_error(error: FastReleaseError) {
  error!("An error has occurred:");

  if let Some(content) = &error.error {
    error!(message = format!("{}", error.message), error = ?content);
  } else {
    error!(message = format!("{}", error.message));
  }

  warn!("Shutting down.");
  std::process::exit(1);
}
