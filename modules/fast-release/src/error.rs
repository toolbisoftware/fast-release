// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::io::Error;
use tracing::{error, warn};

pub struct FastReleaseError {
  pub message: String,
  pub category: Option<String>,
  pub error: Option<Error>,
}

impl std::fmt::Display for FastReleaseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let message: &String = &self.message;
    let error: String = if let Some(value) = &self.error {
      format!("\n{}", value)
    } else {
      "".into()
    };

    write!(f, "{}{}", message, error)
  }
}

//

pub struct FastReleaseErrorBuilder {
  inner: FastReleaseError,
}

impl FastReleaseErrorBuilder {
  pub fn new(message: &str) -> Self {
    Self {
      inner: FastReleaseError {
        message: message.to_string(),
        category: None,
        error: None,
      },
    }
  }

  pub fn category(mut self, category: &str) -> Self {
    self.inner.category = Some(category.to_string());
    self
  }

  pub fn error(mut self, error: Error) -> Self {
    self.inner.error = Some(error);
    self
  }

  pub fn get(self) -> FastReleaseError {
    self.inner
  }
}

//

pub fn soft_panic(error: FastReleaseError) {
  error!("An error has occurred.");
  error!(message = error.message, category = error.category, error = ?error.error);
  warn!("Shutting down.");

  std::process::exit(1);
}
