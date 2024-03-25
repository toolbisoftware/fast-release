// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::{
  builder::CiEnvBuilder,
  util::{env_var_exists, get_env_var},
  CiEnv, CiServices,
};
use std::collections::HashMap;

pub fn detect(env: &HashMap<String, String>) -> bool {
  env_var_exists(env, "GITLAB_CI")
}

pub fn get(env: &HashMap<String, String>) -> CiEnv {
  let ci_project_url: Option<String> = get_env_var(env, "CI_PROJECT_URL");

  let pull_request: Option<String> = get_env_var(env, "CI_MERGE_REQUEST_ID");
  let branch: Option<String> = pull_request
    .as_ref()
    .and_then(|_| get_env_var(env, "CI_MERGE_REQUEST_TARGET_BRANCH_NAME"))
    .or_else(|| get_env_var(env, "CI_COMMIT_REF_NAME"));
  let job: Option<String> = get_env_var(env, "CI_JOB_ID");
  let job_url: Option<String> = job.as_ref().and_then(|j| {
    ci_project_url
      .as_ref()
      .map(|cpu: &String| format!("{}/-/jobs/{}", cpu, j))
  });
  let build: Option<String> = get_env_var(env, "CI_PIPELINE_ID");
  let build_url: Option<String> = build
    .as_ref()
    .and_then(|b: &String| ci_project_url.map(|cpu: String| format!("{}/pipelines/{}", cpu, b)));
  let is_pull_request: bool = pull_request.is_some();

  CiEnvBuilder::new("GitLab CI/CD", CiServices::GitLab)
    .slug(get_env_var(env, "CI_PROJECT_PATH"))
    .root(get_env_var(env, "CI_PROJECT_DIR"))
    .commit(get_env_var(env, "CI_COMMIT_SHA"))
    .tag(get_env_var(env, "CI_COMMIT_TAG"))
    .branch(branch)
    .pull_request(pull_request)
    .pull_request_branch(get_env_var(env, "CI_MERGE_REQUEST_SOURCE_BRANCH_NAME"))
    .job(job)
    .job_url(job_url)
    .build(build)
    .build_url(build_url)
    .is_ci(true)
    .is_pull_request(is_pull_request)
    .get()
}
