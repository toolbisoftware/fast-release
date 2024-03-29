// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Add automatic tests
// TODO Test everything
// TODO Add more services
// TODO Add in-code documentation
// TODO Add the essential traits for the exported structs

mod builder;
mod services;
mod util;

use std::{collections::HashMap, env};

#[derive(Debug)]
pub enum CiServices {
  Git,
  GitHub,
  GitLab,
}

#[derive(Debug)]
pub struct CiEnv {
  pub name: String,
  pub service: CiServices,
  //
  pub slug: Option<String>,
  pub root: Option<String>,
  //
  pub commit: Option<String>,
  pub tag: Option<String>,
  pub branch: Option<String>,
  pub pull_request: Option<String>,
  pub pull_request_branch: Option<String>,
  //
  pub job: Option<String>,
  pub job_url: Option<String>,
  //
  pub build: Option<String>,
  pub build_url: Option<String>,
  //
  pub is_ci: bool,
  pub is_pull_request: bool,
}

fn detect(env: &HashMap<String, String>) -> CiServices {
  if services::github::detect(&env) {
    return CiServices::GitHub;
  }
  if services::gitlab::detect(&env) {
    return CiServices::GitLab;
  }

  return CiServices::Git;
}

pub fn get() -> CiEnv {
  let env: HashMap<String, String> = env::vars().collect();
  let service: CiServices = detect(&env);

  match service {
    CiServices::GitHub => services::github::get(&env),
    CiServices::GitLab => services::gitlab::get(&env),
    CiServices::Git => services::git::get(&env),
  }
}
