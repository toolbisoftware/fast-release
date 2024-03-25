// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{collections::HashMap, process::Command};

pub fn env_var_exists(env: &HashMap<String, String>, variable: &str) -> bool {
  env.get(variable).map_or(false, |v: &String| !v.is_empty())
}

pub fn get_env_var(env: &HashMap<String, String>, variable: &str) -> Option<String> {
  env
    .get(variable)
    .filter(|v: &&String| !v.is_empty())
    .cloned()
}

pub fn get_commit() -> Option<String> {
  let command: Result<Vec<u8>, std::io::Error> = Command::new("git")
    .args(["rev-parse", "HEAD"])
    .output()
    .map(|v: std::process::Output| v.stdout);

  let output: Option<&str> = command
    .as_ref()
    .ok()
    .and_then(|v: &Vec<u8>| std::str::from_utf8(v).ok());

  output.map(|v: &str| v.trim().to_string())
}

pub fn get_branch() -> Option<String> {
  match Command::new("git")
    .args(["rev-parse", "--abbrev-ref", "HEAD"])
    .output()
  {
    Ok(output) => {
      let head_ref: String = std::str::from_utf8(&output.stdout).ok()?.trim().to_string();
      if head_ref == "HEAD" {
        let command: std::process::Output = Command::new("git")
          .args(["show", "-s", "--pretty=%d", "HEAD"])
          .output()
          .ok()?;

        let output: Option<String> = std::str::from_utf8(&command.stdout)
          .ok()?
          .replace(|v| v == '(' || v == ')', "")
          .split(", ")
          .find(|v: &&str| v.starts_with("origin/"))
          .and_then(|v: &str| v.strip_prefix("origin/"))
          .map(|v: &str| v.to_string());

        return output;
      }

      Some(head_ref)
    }
    Err(_) => None,
  }
}
