// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::{parse::parse, Protocol, ProtocolType};
use std::{collections::HashMap, io::Error};

pub struct FromUrl {
  url: String,
  no_commitish: bool,
  no_git_plus: bool,
}

fn is_github_shorthand(url: &str) -> bool {
  let at_idx: Option<usize> = url.find('@');
  let hash_idx: Option<usize> = url.find('#');
  let colon_idx: Option<usize> = url.find(':');
  let first_slash_idx: Option<usize> = url.find('/');
  let second_slash_idx: Option<usize> = first_slash_idx.and_then(|idx: usize| {
    url[(idx + 1)..]
      .find('/')
      .map(|inner_idx: usize| idx + 1 + inner_idx)
  });
  let space_idx: Option<usize> = url.find(char::is_whitespace);

  let does_not_start_with_dot: bool = !url.starts_with('.');
  if !does_not_start_with_dot {
    return false;
  }

  let at_only_after_hash: bool = at_idx
    .and_then(|at_idx: usize| hash_idx.map(|hash_idx: usize| at_idx > hash_idx))
    .unwrap_or(true);
  if !at_only_after_hash {
    return false;
  }

  let colon_only_after_hash: bool = colon_idx
    .and_then(|colon_idx: usize| hash_idx.map(|hash_idx: usize| colon_idx > hash_idx))
    .unwrap_or(true);
  if !colon_only_after_hash {
    return false;
  }

  let second_slash_only_after_hash: bool = second_slash_idx
    .and_then(|second_slash_idx: usize| hash_idx.map(|hash_idx: usize| second_slash_idx > hash_idx))
    .unwrap_or(true);
  if !second_slash_only_after_hash {
    return false;
  }

  let space_only_after_hash: bool = space_idx
    .and_then(|space_idx: usize| hash_idx.map(|hash_idx: usize| space_idx > hash_idx))
    .unwrap_or(true);
  if !space_only_after_hash {
    return false;
  }

  let has_slash: bool = first_slash_idx.map_or(false, |i: usize| i > 0);
  if !has_slash {
    return false;
  }

  let does_not_end_with_slash: bool = hash_idx.map_or(!url.ends_with('/'), |hash_idx: usize| {
    hash_idx > 0 && url.chars().nth(hash_idx - 1) != Some('/')
  });
  if !does_not_end_with_slash {
    return false;
  }

  true
}
