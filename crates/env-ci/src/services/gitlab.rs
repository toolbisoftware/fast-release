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

  let slug: Option<String> = get_env_var(env, "CI_PROJECT_PATH");
  let root: Option<String> = get_env_var(env, "CI_PROJECT_DIR");
  let commit: Option<String> = get_env_var(env, "CI_COMMIT_SHA");
  let tag: Option<String> = get_env_var(env, "CI_COMMIT_TAG");
  let pull_request: Option<String> = get_env_var(env, "CI_MERGE_REQUEST_ID");
  let branch: Option<String> = if pull_request.is_some() {
    get_env_var(env, "CI_MERGE_REQUEST_TARGET_BRANCH_NAME")
  } else {
    get_env_var(env, "CI_COMMIT_REF_NAME")
  };
  let pull_request_branch: Option<String> = get_env_var(env, "CI_MERGE_REQUEST_SOURCE_BRANCH_NAME");
  let job: Option<String> = get_env_var(env, "CI_JOB_ID");
  let job_url: Option<String> = {
    if let Some(job) = &job {
      if let Some(ci_project_url) = &ci_project_url {
        Some(format!("{}/-/jobs/{}", ci_project_url, job));
      }
    }
    None
  };
  let build: Option<String> = get_env_var(env, "CI_PIPELINE_ID");
  let build_url: Option<String> = {
    if let Some(build) = &build {
      if let Some(ci_project_url) = ci_project_url {
        Some(format!("{}/pipelines/{}", ci_project_url, build));
      }
    }
    None
  };
  let is_pull_request: bool = pull_request.is_some();

  CiEnvBuilder::new("GitLab CI/CD", CiServices::GitLab)
    .slug(slug)
    .root(root)
    .commit(commit)
    .tag(tag)
    .branch(branch)
    .pull_request(pull_request)
    .pull_request_branch(pull_request_branch)
    .job(job)
    .job_url(job_url)
    .build(build)
    .build_url(build_url)
    .is_pull_request(is_pull_request)
    .get()
}
