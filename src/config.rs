// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use config::ConfigError;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub token:       String,
  pub admin_roles: Vec<u64>,
}
impl Config {
  #[allow(future_prelude_collision)]
  fn new() -> Result<Self, ConfigError> {
    let mut c = config::Config::default();

    c.merge(config::File::with_name(".dos-bot/config.json"))
      .expect("unable to access configuration file");

    c.try_into()
  }

  /// # Panics
  /// if the configuration file is unable to be accessed.
  #[must_use]
  pub fn get() -> Self { Self::new().unwrap() }
}
