use chrono::Utc;
use eyre::Result;

use super::constructor::*;
use crate::JournalEntry;

impl<'a> Repo<'a> {
  pub fn create(&self, journal_entry: JournalEntry) -> Result<JournalEntry> {
    self.save(journal_entry)
  }

  pub fn update(&self, mut journal_entry: JournalEntry) -> Result<JournalEntry> {
    journal_entry.set_updated_at(Utc::now());
    self.save(journal_entry)
  }

  fn save(&self, journal_entry: JournalEntry) -> Result<JournalEntry> {
    let mut statement = self.connection.prepare(UPSERT_SQL)?;
    statement.execute(journal_entry.to_sql_params())?;

    self.by_pk(journal_entry.id())
  }
}

const UPSERT_SQL: &str = r"
  INSERT INTO journal (id, content, created_at, updated_at)
  VALUES (?1, ?2, ?3, ?4)
  ON CONFLICT (id) DO UPDATE SET
    content = excluded.content,
    updated_at = excluded.updated_at
";
