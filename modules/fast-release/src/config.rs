// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::{
  cli::CliParams,
  constants::{CONFIG_FILE_EXT, CONFIG_FILE_NAME, CONFIG_VERSION},
  error::{FastReleaseError, FastReleaseErrorBuilder},
};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fs::{self, File, OpenOptions},
  io::Read,
  path::{Path, PathBuf},
};
use tracing::{debug, warn};

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFileBranch {
  pre_release: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ConfigBranchEnum {
  Simple(String),
  WithProperties(HashMap<String, Vec<ConfigFileBranch>>),
}

#[derive(Debug, Clone)]
pub struct ConfigBranch {
  pub name: String,
  pub pre_release: bool,
}

//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProject {
  name: String,
  path: String,
  modules: Vec<String>,
}

//

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
  version: u8,
  tag_format: String,
  modules: Vec<String>,
  branches: Vec<ConfigBranchEnum>,
  projects: Vec<ConfigProject>,
}

#[derive(Debug, Clone)]
pub struct Config {
  pub version: u8,
  pub tag_format: String,
  pub modules: Vec<String>,
  pub branches: Vec<ConfigBranch>,
  pub projects: Vec<ConfigProject>,
  pub dry_run: bool,
}

fn find_file(base_path: &Path) -> Option<PathBuf> {
  debug!(
    message = "Trying to find a configuration file.",
    category = "CONFIG"
  );

  let mut files_found: u8 = 0;
  let mut file: Option<(String, PathBuf)> = None;

  for file_name in CONFIG_FILE_NAME {
    for file_ext in CONFIG_FILE_EXT {
      let file_name: String = format!("{}.{}", file_name, file_ext);
      let path: PathBuf = base_path.join(&file_name);

      if fs::metadata(&path).is_ok() {
        if files_found == 0 {
          file = Some((file_name, path));
        }
        files_found = files_found + 1;
      }
    }
  }

  if files_found > 0 {
    let file: (String, PathBuf) = file.unwrap();

    if files_found > 1 {
      warn!(
        message = format!(
          "Found more than one configuration file. Using '{}'.",
          file.0
        ),
        category = "CONFIG"
      )
    } else if files_found == 1 {
      debug!(
        message = format!("Using the configuration file '{}'.", file.0),
        category = "CONFIG"
      );
    }

    return Some(file.1);
  }

  None
}

pub fn get_file(file_path: &Option<String>) -> Result<PathBuf, FastReleaseError> {
  let exe_path: PathBuf = std::env::current_exe().unwrap();
  let base_path: &Path = exe_path.parent().unwrap();

  let file: Option<PathBuf> = {
    let get_file: Option<PathBuf> = {
      if let Some(file_path) = file_path {
        debug!(
          message = format!(
            "Trying to find the custom configuration file '{}'.",
            file_path
          ),
          category = "CONFIG"
        );

        let path: PathBuf = base_path.join(&file_path);
        if fs::metadata(&path).is_ok() {
          Some(path);
        };

        warn!(
          message = format!(
            "Couldn't find the custom configuration file '{}'.",
            file_path
          ),
          category = "CONFIG"
        );
      }

      None
    };

    let get_file: Option<PathBuf> = {
      if get_file.is_none() {
        find_file(base_path)
      } else {
        get_file
      }
    };

    get_file
  };

  match file {
    Some(file) => Ok(file),
    None => Err(
      FastReleaseErrorBuilder::new("Couldn't find a configuration file.")
        .category("CONFIG")
        .get(),
    ),
  }
}

fn read_file(file_path: PathBuf) -> Result<ConfigFile, FastReleaseError> {
  let mut file: File = match OpenOptions::new().read(true).open(file_path) {
    Ok(file) => file,
    Err(error) => {
      return Err(
        FastReleaseErrorBuilder::new("Failed to open the configuration file.")
          .category("CONFIG")
          .error(error)
          .get(),
      )
    }
  };

  let content: String = {
    let mut data: String = String::new();
    let _ = file.read_to_string(&mut data);
    data
  };

  let parse: ConfigFile = match serde_yaml::from_str(&content) {
    Ok(content) => content,
    Err(_) => {
      return Err(
        FastReleaseErrorBuilder::new(
          "Failed to parse the configuration file. The file might be wrongly formatted.",
        )
        .category("CONFIG")
        .get(),
      );
    }
  };

  Ok(parse)
}

fn validate_and_transform_config(
  file_config: ConfigFile,
  cli_params: &CliParams,
) -> Result<Config, FastReleaseError> {
  fn version(version: u8) -> u8 {
    if version != CONFIG_VERSION {
      // TODO Handle outdated configuration files
      warn!(
        message = "The configuration file is on version '{}'. It should be on version '{}'.",
        category = "CONFIG"
      )
    }

    version
  }

  // TODO Handle tag format
  fn tag_format(tag_format: String) -> String {
    tag_format
  }

  // TODO Handle modules
  fn modules(modules: Vec<String>) -> Vec<String> {
    modules
  }

  fn branches(branches: Vec<ConfigBranchEnum>) -> Result<Vec<ConfigBranch>, FastReleaseError> {
    if branches.len() == 0 {
      return Err(
        FastReleaseErrorBuilder::new(
          "There are no branches on the configuration file. There must be at least one.",
        )
        .category("CONFIG")
        .get(),
      );
    }

    let mut result: Vec<ConfigBranch> = Vec::new();
    for branch in branches {
      match branch {
        ConfigBranchEnum::Simple(name) => result.push(ConfigBranch {
          name,
          pre_release: false,
        }),
        ConfigBranchEnum::WithProperties(properties) => {
          for (name, branches) in properties {
            for branch in branches {
              result.push(ConfigBranch {
                name,
                pre_release: branch.pre_release.unwrap_or(false),
              });
              break;
            }
            break;
          }
        }
      }
    }

    Ok(result)
  }

  fn projects(projects: Vec<ConfigProject>) -> Result<Vec<ConfigProject>, FastReleaseError> {
    if projects.len() == 0 {
      return Err(
        FastReleaseErrorBuilder::new(
          "There are no projects on the configuration file. There must be at least one.",
        )
        .category("CONFIG")
        .get(),
      );
    }

    Ok(projects)
  }

  fn dry_run(dry_run: bool) -> bool {
    dry_run
  }

  Ok(Config {
    version: version(file_config.version),
    tag_format: tag_format(file_config.tag_format),
    modules: modules(file_config.modules),
    branches: branches(file_config.branches)?,
    projects: projects(file_config.projects)?,
    dry_run: dry_run(cli_params.dry_run),
  })
}

pub fn get(cli_params: &CliParams) -> Result<Config, FastReleaseError> {
  let get_file: PathBuf = get_file(&cli_params.config_file_path)?;
  let read_file: ConfigFile = read_file(get_file)?;
  let config: Config = validate_and_transform_config(read_file, cli_params)?;

  Ok(config)
}
