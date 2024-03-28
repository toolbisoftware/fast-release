// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::protocol::{self, Protocol, ProtocolType};
use std::{
  collections::HashMap,
  io::{Error, ErrorKind},
};
use url::Url;

fn last_index_of_before(string: &str, ch: char, before_char: char) -> Option<usize> {
  string
    .find(before_char)
    .map_or(None, |s| string[..s].rfind(ch))
}

fn correct_protocol(
  url: &str,
  protocols: &HashMap<ProtocolType, Protocol>,
) -> Result<String, Error> {
  if let Some(colon_idx) = url.find(':') {
    let protocol_str: &str = &url[..=colon_idx];
    let protocol: ProtocolType = protocol::type_from_str(protocol_str)
      .ok_or_else(|| Error::new(ErrorKind::Other, "Invalid protocol."))?;
    if protocols.contains_key(&protocol) {
      return Ok(url.to_string());
    }

    if let Some(at_idx) = url.find('@') {
      if at_idx > colon_idx {
        return Ok(format!("git+ssh://{}", url));
      } else {
        return Ok(url.to_string());
      }
    }

    if let Some(double_slash_idx) = url.find("//") {
      if double_slash_idx == colon_idx + 1 {
        return Ok(url.to_string());
      }
    }

    return Ok(format!(
      "{}//{}",
      &url[..=colon_idx],
      &url[(colon_idx + 1)..]
    ));
  }

  Ok(url.to_string())
}

fn correct_url(url: &str) -> String {
  let last_colon_idx: Option<usize> = last_index_of_before(url, ':', '#');
  let at_idx: Option<usize> = last_index_of_before(url, '@', '#');

  if let (Some(last_colon_idx), Some(at_idx)) = (last_colon_idx, at_idx) {
    if last_colon_idx > at_idx {
      let mut result = String::from(&url[(last_colon_idx + 1)..]);
      result.push('/');
      result.push_str(&url[(last_colon_idx + 1)..]); // ?

      return result;
    }
  }

  if last_index_of_before(url, ':', '#').is_none() && url.find("//").is_none() {
    return format!("git+ssh://{}", url);
  }

  url.to_string()
}

pub fn parse(url: &str, protocols: Option<&HashMap<ProtocolType, Protocol>>) -> Result<Url, Error> {
  let with_protocol: String =
    protocols.map_or_else(|| Ok(url.to_string()), |p| correct_protocol(url, p))?;

  Url::parse(&with_protocol)
    .or_else(|_| Url::parse(&correct_url(&with_protocol)))
    .or_else(|_| Err(Error::new(ErrorKind::Other, "Failed to parse the URL.")))
}
