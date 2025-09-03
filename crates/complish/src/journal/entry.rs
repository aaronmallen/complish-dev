use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize, Setters)]
pub struct Entry {
  #[getset(get = "pub", set = "pub")]
  content: String,
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: DateTime<Utc>,
}

impl Entry {
  pub fn new(content: impl Into<String>) -> Self {
    let now = Utc::now();

    Self {
      content: content.into(),
      created_at: now,
      id: cuid2::create_id(),
      updated_at: now,
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 4] {
    [&self.id, &self.content, &self.created_at, &self.updated_at]
  }
}

impl TryFrom<&Row<'_>> for Entry {
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
