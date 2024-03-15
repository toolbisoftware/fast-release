// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::{
  builder::CiEnvBuilder,
  util::{env_var_exists, get_env_var},
  CiEnv, CiServices,
};
use regex::Regex;
use std::{collections::HashMap, fs};

pub fn detect(env: &HashMap<String, String>) -> bool {
  env_var_exists(env, "GITHUB_ACTIONS")
}

fn parse_branch(branch: &str) -> Option<String> {
  if let Some(captures) = Regex::new(r"^(?:refs\/heads\/)?(?P<branch>.+)$")
    .unwrap()
    .captures(branch)
  {
    Some(captures["branch"].to_string()) // ! ?
  } else {
    None
  }
}

fn get_pull_request_event(env: &HashMap<String, String>) -> Option<(String, String)> {
  if let Some(event_path) = get_env_var(env, "GITHUB_EVENT_PATH") {
    if let Ok(event_content) = fs::read_to_string(event_path) {
      if let Ok(event) = serde_json::from_str::<serde_json::Value>(&event_content) {
        if let Some(pull_request) = event.get("pull_request") {
          if let (Some(base), Some(number)) = (
            pull_request.get("base"),
            pull_request.get("number").and_then(|num| num.as_u64()),
          ) {
            if let Some(ref_val) = base
              .get("ref")
              .and_then(|ref_val| parse_branch(ref_val.as_str()?))
            {
              return Some((ref_val, number.to_string()));
            }
          }
        };
      }
    }
  };

  None
}

pub fn get(env: &HashMap<String, String>) -> CiEnv {
  let github_event_name: Option<String> = get_env_var(env, "GITHUB_EVENT_NAME");
  let pull_request_event: Option<(String, String)> = get_pull_request_event(env);
  let (pre_ref, pre_id) = if let Some((ref_val, number)) = pull_request_event {
    (Some(ref_val), Some(number))
  } else {
    (None, None)
  };

  let slug: Option<String> = get_env_var(env, "GITHUB_REPOSITORY");
  let root: Option<String> = get_env_var(env, "GITHUB_WORKSPACE");
  let commit: Option<String> = get_env_var(env, "GITHUB_SHA");
  let branch: Option<String> = {
    let name: Option<String> = pre_ref.or_else(|| {
      github_event_name
        .as_ref()
        .filter(|&name| name == "pull_request_target")
        .and_then(|_| pre_id.as_ref())
        .map(|pre_id| format!("refs/pull/{}/merge", pre_id))
        .or_else(|| get_env_var(env, "GITHUB_REF"))
    });

    let parse: Option<String> = {
      if let Some(name) = name {
        parse_branch(&name);
      }
      None
    };

    parse
  };
  let pull_request: Option<String> = pre_id;
  let build: Option<String> = get_env_var(env, "GITHUB_RUN_ID");
  let is_pull_request: bool = if let Some(github_event_name) = github_event_name {
    github_event_name == "pull_request" || github_event_name == "pull_request_target"
  } else {
    false
  };
  let pull_request_branch: Option<String> = if is_pull_request {
    branch.clone()
  } else {
    None
  };

  CiEnvBuilder::new("GitHub Actions", CiServices::GitHub)
    .slug(slug)
    .root(root)
    .commit(commit)
    .branch(branch)
    .pull_request(pull_request)
    .pull_request_branch(pull_request_branch)
    .build(build)
    .is_ci(true)
    .is_pull_request(is_pull_request)
    .get()
}
