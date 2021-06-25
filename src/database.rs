// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use simple_error::SimpleResult;
#[allow(clippy::wildcard_imports)]
use simplebase::engine::*;

pub struct Database(pub RecordData);
impl Database {
  #[must_use]
  pub fn new() -> Self {
    if std::path::Path::new(".dos-bot/db.txt").exists() {
      Self(load_hash_database(".dos-bot/db.txt"))
    } else {
      Self(new_empty_database())
    }
  }

  pub fn create_reaction_role(&mut self, message: u64, role: u64) {
    self
      .0
      .add_record_with_key(message.to_string(), role.to_string());

    self.0.save_database(".dos-bot/db.txt");
  }

  /// # Panics
  /// if the record index is unable to be parsed.
  pub fn remove_reaction_role(&mut self, message: u64) {
    self.0.delete_record(
      self.0.find_key(message.to_string().as_str())[0]
        .parse::<usize>()
        .unwrap(),
    );

    self.0.save_database(".dos-bot/db.txt");
  }

  /// # Errors
  /// if no record is found for the given `message`.
  ///
  /// # Panics
  /// if the record is unable to be parsed.
  pub fn get_reaction_role(&self, message: u64) -> SimpleResult<u64> {
    let record = self.0.find_key(message.to_string().as_str());
    if record.is_empty() {
      simple_error::bail!("no record found");
    }

    Ok(record[1].clone().parse::<u64>().unwrap())
  }

  #[must_use]
  pub fn count(&self) -> String { self.0.record_counter.to_string() }
}
impl Default for Database {
  fn default() -> Self { Self::new() }
}
