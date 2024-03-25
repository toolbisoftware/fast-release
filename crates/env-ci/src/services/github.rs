// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use super::Service;
use crate::{
  builder::CiEnvBuilder,
  util::{env_var_exists, get_env_var},
  CiEnv, CiServices,
};
use regex::Regex;
use std::{collections::HashMap, fs};

fn parse_branch(branch: &str) -> Option<String> {
  if let Some(captures) = Regex::new(r"^(?:refs\/heads\/)?(?P<branch>.+)$")
    .unwrap()
    .captures(branch)
  {
    Some(captures["branch"].to_string())
  } else {
    None
  }
}

struct PullRequestEvent {
  pub ref_val: Option<String>,
  pub id: Option<String>,
}

fn get_pull_request_event(env: &HashMap<String, String>) -> PullRequestEvent {
  let mut ref_val: Option<String> = None;
  let mut id: Option<String> = None;

  if let Some(path) = get_env_var(env, "GITHUB_EVENT_PATH") {
    if let Ok(content) = fs::read_to_string(path) {
      if let Ok(event) = serde_json::from_str::<serde_json::Value>(&content) {
        if let Some(pull_request) = event.get("pull_request") {
          if let Some(base) = pull_request.get("base") {
            if let Some(v) = base
              .get("ref")
              .and_then(|v: &serde_json::Value| parse_branch(v.as_str()?))
            {
              ref_val = Some(v);
            }
          }
          if let Some(number) = pull_request
            .get("number")
            .and_then(|num: &serde_json::Value| num.as_u64())
          {
            id = Some(number.to_string());
          }
        }
      }
    }
  }

  PullRequestEvent { ref_val, id }
}

struct GitHub;

impl Service for GitHub {
  fn detect(env: &HashMap<String, String>) -> bool {
    env_var_exists(env, "GITHUB_ACTIONS")
  }

  fn get(env: &HashMap<String, String>) -> CiEnv {
    let github_event_name: Option<String> = get_env_var(env, "GITHUB_EVENT_NAME");
    let pull_request_event: PullRequestEvent = get_pull_request_event(env);
    let pre_ref: Option<String> = pull_request_event.ref_val;
    let pre_id: Option<String> = pull_request_event.id;

    let branch: Option<String> = {
      let name: Option<String> = pre_ref.or_else(|| {
        github_event_name
          .as_ref()
          .filter(|&name| name == "pull_request_target")
          .and_then(|_| pre_id.as_ref())
          .map(|pid: &String| format!("refs/pull/{}/merge", pid))
          .or_else(|| get_env_var(env, "GITHUB_REF"))
      });

      name.and_then(|v: String| parse_branch(&v)).or_else(|| None)
    };

    let pull_request: Option<String> = pre_id;

    let is_pull_request: bool = github_event_name
      .map(|v: String| v == "pull_request" || v == "pull_request_target")
      .unwrap_or(false);

    let pull_request_branch: Option<String> =
      is_pull_request.then(|| branch.clone()).unwrap_or(None);

    CiEnvBuilder::new("GitHub Actions", CiServices::GitHub)
      .slug(get_env_var(env, "GITHUB_REPOSITORY"))
      .root(get_env_var(env, "GITHUB_WORKSPACE"))
      .commit(get_env_var(env, "GITHUB_SHA"))
      .branch(branch)
      .pull_request(pull_request)
      .pull_request_branch(pull_request_branch)
      .build(get_env_var(env, "GITHUB_RUN_ID"))
      .is_ci(true)
      .is_pull_request(is_pull_request)
      .get()
  }
}
