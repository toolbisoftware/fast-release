// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use log::info;

mod constants;
mod util;

fn setup_logger() {
  util::logger::init(log::LevelFilter::Info);
}

fn main() {
  setup_logger();

  info!("Running fast-release {}.", constants::VERSION);
}
