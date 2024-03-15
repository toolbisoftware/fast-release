// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::{
  builder::CiEnvBuilder,
  util::{env_var_exists, get_branch, get_commit},
  CiEnv, CiServices,
};
use std::collections::HashMap;

pub fn get(env: &HashMap<String, String>) -> CiEnv {
  let commit: Option<String> = get_commit();
  let branch: Option<String> = get_branch();
  let is_ci: bool = env_var_exists(env, "CI");

  CiEnvBuilder::new("Git", CiServices::Git)
    .commit(commit)
    .branch(branch)
    .is_ci(is_ci)
    .get()
}
