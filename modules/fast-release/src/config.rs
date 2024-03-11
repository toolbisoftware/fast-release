// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::{
  cli::{self, CliParams},
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProject {
  name: String,
  path: String,
  modules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawConfig {
  version: u8,
  tag_format: String,
  modules: Vec<String>,
  branches: Vec<ConfigBranchEnum>,
  project: Option<ConfigProject>,
  projects: Option<Vec<ConfigProject>>,
}

#[derive(Debug, Clone)]
pub struct Config {
  pub version: u8,
  pub tag_format: String,
  pub modules: Vec<String>,
  pub branches: Vec<Branch>,
  pub project: Option<ConfigProject>,
  pub projects: Option<Vec<ConfigProject>>,
  pub dry_run: bool,
}

fn find_file(path_override: Option<String>) -> Result<PathBuf, FastReleaseError<'static>> {
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

  if let Some(path_override) = path_override {
    let path = path.join(&path_override);

    if fs::metadata(&path).is_ok() {
      debug!(
        category = "CONFIG",
        message = format!("Found the configuration file '{}'.", path_override)
      );
      return Ok(path.into());
    }

    Err(FastReleaseError {
      message: "Couldn't find the configuration file.",
      category: Some("CONFIG"),
      error: None,
    })
  } else {
    debug!(
      category = "CONFIG",
      message = "Trying to find a configuration file."
    );

    for file_name in CONFIG_FILE_NAMES {
      for file_ext in CONFIG_FILE_EXTS {
        let file_name: String = format!("{}.{}", file_name, file_ext);
        let path: PathBuf = path.join(&file_name);

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
}

fn get_config(path: PathBuf) -> Result<RawConfig, FastReleaseError<'static>> {
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

  let parse_content: RawConfig = match serde_yaml::from_str(&content) {
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

fn convert_branches(config_branches: Vec<ConfigBranchEnum>) -> Vec<Branch> {
  if config_branches.len() == 0 {
    throw_error(FastReleaseError {
      message: "There are no branches on the configuration file.",
      category: Some("CONFIG"),
      error: None,
    })
  }

  let mut converted_branches: Vec<Branch> = Vec::new();
  for branch in config_branches {
    let _ = match branch {
      ConfigBranchEnum::Simple(name) => converted_branches.push(Branch {
        name,
        pre_release: false,
      }),
      ConfigBranchEnum::WithProperties(properties) => {
        for (name, branches) in properties {
          for branch in branches {
            let mut pre_release: bool = false;
            if let Some(value) = branch.pre_release {
              pre_release = value;
            }

            converted_branches.push(Branch {
              name: name.to_owned(),
              pre_release,
            });

            break;
          }

          break;
        }
      }
    };
  }

  converted_branches
}

fn transform_config(raw_config: RawConfig, cli_params: CliParams) -> Config {
  let dry_run: bool = cli_params.dry_run;

  Config {
    version: raw_config.version,
    tag_format: raw_config.tag_format,
    modules: raw_config.modules,
    branches: convert_branches(raw_config.branches),
    project: raw_config.project,
    projects: raw_config.projects,
    dry_run,
  }
}

pub fn get() -> Result<Config, FastReleaseError<'static>> {
  let get_cli_params: CliParams = cli::get();
  let find_file: PathBuf = find_file(get_cli_params.config_file_path.clone())?;
  let get_config: RawConfig = get_config(find_file)?;
  let transform_config: Config = transform_config(get_config, get_cli_params);

  Ok(transform_config)
}
