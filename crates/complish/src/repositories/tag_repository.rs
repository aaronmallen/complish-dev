use eyre::Result;
use rusqlite::{Connection, types::ToSql};

use crate::entities::Tag;

pub struct TagRepository<'a> {
  connection: &'a Connection,
}

impl<'a> TagRepository<'a> {
  pub fn new(connection: &'a Connection) -> Self {
    Self {
      connection,
    }
  }

  pub fn by_label(&self, label: impl Into<String>) -> Result<Tag> {
    self.find_by_sql(TAG_BY_LABEL_SQL, [&label.into()])
  }

  pub fn by_pk(&self, id: impl Into<String>) -> Result<Tag> {
    self.find_by_sql(TAG_BY_PK_SQL, [&id.into()])
  }

  pub fn find_or_create(&self, label: impl Into<String>) -> Result<Tag> {
    let label = label.into();

    match self.by_label(&label) {
      Ok(tag) => Ok(tag),
      Err(_) => {
        let tag = Tag::new(&label);
        self
          .connection
          .execute(INSERT_TAG_SQL, tag.to_sql_params())?;
        Ok(tag)
      }
    }
  }

  pub fn save(&self, tag: &Tag) -> Result<Tag> {
    self
      .connection
      .execute(UPSERT_TAG_SQL, tag.to_sql_params())?;
    self.by_pk(tag.id())
  }

  fn find_by_sql(&self, sql: &str, params: [&dyn ToSql; 1]) -> Result<Tag> {
    let mut statement = self.connection.prepare(sql)?;
    let tag = statement.query_row(params, |row| Tag::try_from(row))?;
    Ok(tag)
  }
}

const INSERT_TAG_SQL: &str = r"
  INSERT INTO tags (id, label, color, created_at, updated_at)
  VALUES (?1, ?2, ?3, ?4, ?5)
";

const TAG_BY_LABEL_SQL: &str = "SELECT * FROM tags WHERE label = ?1";

const TAG_BY_PK_SQL: &str = "SELECT * FROM tags WHERE id = ?1";

const UPSERT_TAG_SQL: &str = r"
  INSERT INTO tags (id, label, color, created_at, updated_at)
  VALUES (?1, ?2, ?3, ?4, ?5)
  ON CONFLICT (id) DO UPDATE SET
    label = excluded.label,
    color = excluded.color,
    updated_at = excluded.updated_at
";
