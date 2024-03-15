// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::{CiEnv, CiServices};

pub struct CiEnvBuilder {
  inner: CiEnv,
}

impl CiEnvBuilder {
  pub fn new(name: &str, service: CiServices) -> Self {
    Self {
      inner: CiEnv {
        name: name.to_string(),
        service,
        slug: None,
        root: None,
        commit: None,
        tag: None,
        branch: None,
        pull_request: None,
        pull_request_branch: None,
        job: None,
        job_url: None,
        build: None,
        build_url: None,
        is_ci: false,
        is_pull_request: false,
      },
    }
  }

  pub fn slug(mut self, slug: Option<String>) -> Self {
    self.inner.slug = slug;
    self
  }

  pub fn root(mut self, root: Option<String>) -> Self {
    self.inner.root = root;
    self
  }

  pub fn commit(mut self, commit: Option<String>) -> Self {
    self.inner.commit = commit;
    self
  }

  pub fn tag(mut self, tag: Option<String>) -> Self {
    self.inner.tag = tag;
    self
  }

  pub fn branch(mut self, branch: Option<String>) -> Self {
    self.inner.branch = branch;
    self
  }

  pub fn pull_request(mut self, pull_request: Option<String>) -> Self {
    self.inner.pull_request = pull_request;
    self
  }

  pub fn pull_request_branch(mut self, pull_request_branch: Option<String>) -> Self {
    self.inner.pull_request_branch = pull_request_branch;
    self
  }

  pub fn job(mut self, job: Option<String>) -> Self {
    self.inner.job = job;
    self
  }

  pub fn job_url(mut self, job_url: Option<String>) -> Self {
    self.inner.job_url = job_url;
    self
  }

  pub fn build(mut self, build: Option<String>) -> Self {
    self.inner.build = build;
    self
  }

  pub fn build_url(mut self, build_url: Option<String>) -> Self {
    self.inner.build_url = build_url;
    self
  }

  pub fn is_ci(mut self, is_ci: bool) -> Self {
    self.inner.is_ci = is_ci;
    self
  }

  pub fn is_pull_request(mut self, is_pull_request: bool) -> Self {
    self.inner.is_pull_request = is_pull_request;
    self
  }

  pub fn get(self) -> CiEnv {
    self.inner
  }
}
