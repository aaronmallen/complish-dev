use chrono::{DateTime, Utc};
use cuid::cuid2;
use getset::Getters;
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct JournalEntry {
  #[get = "pub"]
  content: String,
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get = "pub"]
  id: String,
  #[get = "pub"]
  updated_at: DateTime<Utc>,
}

impl JournalEntry {
  pub fn new(content: impl Into<String>) -> Self {
    let now = Utc::now();

    Self {
      content: content.into(),
      created_at: now,
      id: cuid2(),
      updated_at: now,
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 4] {
    [&self.id, &self.content, &self.created_at, &self.updated_at]
  }

  pub fn touch(&mut self) {
    self.updated_at = Utc::now();
  }

  pub fn update_content(&mut self, content: impl Into<String>) {
    self.content = content.into();
    self.touch();
  }
}

impl TryFrom<&Row<'_>> for JournalEntry {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      content: row.get("content")?,
      created_at: row.get("created_at")?,
      id: row.get("id")?,
      updated_at: row.get("updated_at")?,
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod to_sql_params {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_returns_the_journal_entry_fields_as_sql_params() {
      let connection = Connection::open_in_memory().unwrap();
      connection
        .execute(
          r"
          CREATE TABLE journal_entries (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL
          )
        ",
          [],
        )
        .unwrap();

      let journal_entry = JournalEntry::new("a test entry");

      let result = connection.execute(
        r"
          INSERT INTO journal_entries (id, content, created_at, updated_at)
          VALUES (?1, ?2, ?3, ?4)
        ",
        journal_entry.to_sql_params(),
      );

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), 1);
    }
  }

  mod touch {
    use pretty_assertions::assert_ne;

    use super::*;

    #[test]
    fn it_updates_the_updated_at_field() {
      let mut journal_entry = JournalEntry::new("a test entry");
      let old_updated_at = Utc::now() - chrono::Duration::days(2);
      journal_entry.updated_at = old_updated_at;
      journal_entry.touch();

      assert_ne!(journal_entry.updated_at(), &old_updated_at);
      assert!(journal_entry.updated_at() > &old_updated_at);
    }
  }

  mod try_from_row {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_returns_a_journal_entry_from_a_row() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection
        .prepare(
          r"
        SELECT
          'jgnw417dy7mopsp0t785nzup' AS id,
          'a test entry' AS content,
          datetime('now', 'utc') AS created_at,
          datetime('now', 'utc') AS updated_at
      ",
        )
        .unwrap();

      let journal_entry = statement
        .query_row([], |row| JournalEntry::try_from(row))
        .unwrap();

      assert_eq!(journal_entry.id(), "jgnw417dy7mopsp0t785nzup");
      assert_eq!(journal_entry.content(), "a test entry");
    }
  }

  mod update_content {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_content_of_the_journal_entry() {
      let mut journal_entry = JournalEntry::new("a test entry");
      journal_entry.update_content("a new test entry");

      assert_eq!(journal_entry.content(), "a new test entry");
    }
  }
}
