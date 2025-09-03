use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize, Setters)]
pub struct Note {
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[getset(get = "pub", set = "pub")]
  content: String,
  #[get = "pub"]
  id: String,
  #[get = "pub"]
  task_id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: DateTime<Utc>,
}

impl Note {
  pub fn new(task_id: impl Into<String>, content: impl Into<String>) -> Self {
    let now = Utc::now();
    Self {
      created_at: now,
      content: content.into(),
      id: cuid2::create_id(),
      task_id: task_id.into(),
      updated_at: now,
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 5] {
    [
      &self.id,
      &self.task_id,
      &self.content,
      &self.created_at,
      &self.updated_at,
    ]
  }
}

impl TryFrom<&Row<'_>> for Note {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      created_at: row.get("created_at")?,
      content: row.get("content")?,
      id: row.get("id")?,
      task_id: row.get("task_id")?,
      updated_at: row.get("updated_at")?,
    })
  }
}
