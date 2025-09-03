use eyre::Result;
use rusqlite::types::ToSql;

use super::constructor::*;
use crate::Tag;

impl<'a> Repo<'a> {
  pub fn all(&self) -> Result<Vec<Tag>> {
    let mut statement = self.connection.prepare(ALL_SQL)?;
    statement
      .query_map([], |row| Tag::try_from(row))?
      .map(|r| r.map_err(Into::into))
      .collect()
  }

  pub fn by_label(&self, label: impl Into<String>) -> Result<Tag> {
    self.by_sql(BY_LABEL_SQL, [&label.into()])
  }

  pub fn by_pk(&self, id: impl Into<String>) -> Result<Tag> {
    self.by_sql(BY_PK_SQL, [&id.into()])
  }

  fn by_sql(&self, sql: &str, params: [&dyn ToSql; 1]) -> Result<Tag> {
    let mut statement = self.connection.prepare(sql)?;
    statement
      .query_row(params, |row| Tag::try_from(row))
      .map_err(Into::into)
  }
}

const ALL_SQL: &str = "SELECT * FROM tags";

const BY_LABEL_SQL: &str = "SELECT * FROM tags WHERE label = ?1 LIMIT 1";

const BY_PK_SQL: &str = "SELECT * FROM tags WHERE id = ?1 LIMIT 1";
