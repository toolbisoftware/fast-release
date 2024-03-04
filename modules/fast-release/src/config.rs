// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::{
  constants::{CONFIG_FILE_EXTS, CONFIG_FILE_NAMES},
  error::{throw_error, FastReleaseError},
  git::branch::Branch,
};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fs::{self, File},
  io::{Error, Read},
  path::{Path, PathBuf},
};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigBranch {
  pre_release: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ConfigBranchEnum {
  Simple(String),
  WithProperties(HashMap<String, Vec<ConfigBranch>>),
}

// TODO Add the settings inside of each of the declared modules for simplicity.
#[derive(Debug, Serialize, Deserialize)]
struct ConfigProject {
  name: String,
  path: String,
  modules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawConfig {
  version: u8,
  modules: Vec<String>,
  branches: Vec<ConfigBranchEnum>,
  project: Option<ConfigProject>,
  projects: Option<Vec<ConfigProject>>,
}

#[derive(Debug, Clone)]
pub struct Config {
  version: u8,
}

fn find_file() -> Result<PathBuf, FastReleaseError<'static>> {
  debug!(
    category = "CONFIG",
    message = "Trying to find a configuration file."
  );

  let path: PathBuf = std::env::current_exe().map_err(|error: Error| FastReleaseError {
    message: "Failed to get the executable's path.",
    category: Some("CONFIG"),
    error: Some(error),
  })?;
  let path: &Path = match path.parent() {
    Some(path) => path,
    None => {
      return Err(FastReleaseError {
        message: "Failed to get the executable's directory.",
        category: Some("CONFIG"),
        error: None,
      });
    }
  };

  for file_name in CONFIG_FILE_NAMES {
    for file_ext in CONFIG_FILE_EXTS {
      let file_name = format!("{}.{}", file_name, file_ext);
      let path = path.join(&file_name);

      if fs::metadata(&path).is_ok() {
        debug!(
          category = "CONFIG",
          message = format!("Found the configuration file '{}'.", file_name)
        );
        return Ok(path.into());
      }
    }
  }

  Err(FastReleaseError {
    message: "Couldn't find a configuration file.",
    category: Some("CONFIG"),
    error: None,
  })
}

fn get_config(path: PathBuf) -> Result<Config, FastReleaseError<'static>> {
  let mut file: File = match std::fs::OpenOptions::new().read(true).open(path) {
    Ok(file) => file,
    Err(error) => {
      return Err(FastReleaseError {
        message: "Failed to open the configuration file.",
        category: Some("CONFIG"),
        error: Some(error),
      })
    }
  };

  let content: String = {
    let mut content: String = String::new();
    let _ = file.read_to_string(&mut content);

    content
  };

  let parse_content: Config = match serde_yaml::from_str(&content) {
    Ok(content) => content,
    Err(_) => {
      return Err(FastReleaseError {
        message: "Failed to parse the configuration file.",
        category: Some("CONFIG"),
        error: None, // ! Why, serde? Why?
      });
    }
  };

  Ok(parse_content)
}

pub fn get() -> Result<Config, FastReleaseError<'static>> {
  let find_file = find_file()?;
  let get_config = get_config(find_file)?;

  Ok(get_config)
}
