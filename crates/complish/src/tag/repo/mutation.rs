use chrono::Utc;
use eyre::Result;

use super::constructor::*;
use crate::Tag;

impl<'a> Repo<'a> {
  pub fn create(&self, tag: Tag) -> Result<Tag> {
    self.save(tag)
  }

  pub fn update(&self, mut tag: Tag) -> Result<Tag> {
    tag.set_updated_at(Utc::now());
    self.save(tag)
  }

  fn save(&self, tag: Tag) -> Result<Tag> {
    let mut statement = self.connection.prepare(UPSERT_SQL)?;
    statement.execute(tag.to_sql_params())?;
    self.by_pk(tag.id())
  }
}

const UPSERT_SQL: &str = r"
  INSERT INTO tags (id, label, created_at, updated_at)
  VALUES (?1, ?2, ?3, ?4)
  ON CONFLICT (id) DO UPDATE SET
    label = excluded.label,
    updated_at = excluded.updated_at
";
