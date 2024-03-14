// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::collections::HashMap;

pub fn env_var_exists(env: &HashMap<String, String>, variable: &str) -> bool {
  env.get(variable).map_or(false, |v| !v.is_empty())
}

pub fn get_env_var(env: &HashMap<String, String>, variable: &str) -> Option<String> {
  if let Some(value) = env.get(variable) {
    if !value.is_empty() {
      return Some(value.to_owned());
    }
  };

  None
}
