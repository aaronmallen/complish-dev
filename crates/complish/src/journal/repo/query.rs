use chrono::{DateTime, Utc};
use eyre::Result;

use super::constructor::Repo;
use crate::JournalEntry;

impl Repo<'_> {
  pub fn all(&self) -> Result<Vec<JournalEntry>> {
    let mut statement = self.connection.prepare(ALL_SQL)?;
    statement
      .query_map([], |row| JournalEntry::try_from(row))?
      .map(|r| r.map_err(Into::into))
      .collect()
  }

  pub fn all_since(&self, since: DateTime<Utc>) -> Result<Vec<JournalEntry>> {
    let mut statement = self.connection.prepare(ALL_SINCE_SQL)?;
    statement
      .query_map([since], |row| JournalEntry::try_from(row))?
      .map(|r| r.map_err(Into::into))
      .collect()
  }

  pub fn by_pk(&self, id: impl Into<String>) -> Result<JournalEntry> {
    let mut statement = self.connection.prepare(BY_PK_SQL)?;
    statement
      .query_row([id.into()], |row| JournalEntry::try_from(row))
      .map_err(Into::into)
  }
}

const ALL_SQL: &str = "SELECT * FROM journal";

const ALL_SINCE_SQL: &str = "SELECT * FROM journal WHERE created_at >= ?1";

const BY_PK_SQL: &str = "SELECT * FROM journal WHERE id = ?1";
