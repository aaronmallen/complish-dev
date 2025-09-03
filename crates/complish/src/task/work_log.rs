use chrono::{DateTime, Utc};
use getset::{CloneGetters, Getters, Setters};
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

#[derive(Clone, CloneGetters, Debug, Deserialize, Eq, Getters, PartialEq, Serialize, Setters)]
pub struct WorkLog {
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[getset(get_clone = "pub", set = "pub")]
  ended_at: Option<DateTime<Utc>>,
  #[get = "pub"]
  id: String,
  #[getset(get_clone = "pub", set = "pub")]
  note: Option<String>,
  #[getset(get_clone = "pub", set = "pub")]
  source: Option<String>,
  #[getset(get = "pub", set = "pub")]
  started_at: DateTime<Utc>,
  #[get = "pub"]
  task_id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: DateTime<Utc>,
}

impl WorkLog {
  pub fn new(task_id: impl Into<String>) -> Self {
    let now = Utc::now();

    Self {
      created_at: now,
      ended_at: None,
      id: cuid2::create_id(),
      note: None,
      source: None,
      started_at: now,
      task_id: task_id.into(),
      updated_at: now,
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 8] {
    [
      &self.id,
      &self.task_id,
      &self.note,
      &self.source,
      &self.started_at,
      &self.ended_at,
      &self.created_at,
      &self.updated_at,
    ]
  }

  pub fn with_end_date(mut self, ended_at: DateTime<Utc>) -> Self {
    self.ended_at = Some(ended_at);
    self
  }

  pub fn with_note(mut self, note: impl Into<String>) -> Self {
    self.note = Some(note.into());
    self
  }

  pub fn with_source(mut self, source: impl Into<String>) -> Self {
    self.source = Some(source.into());
    self
  }

  pub fn with_start_date(mut self, started_at: DateTime<Utc>) -> Self {
    self.started_at = started_at;
    self
  }
}

impl TryFrom<&Row<'_>> for WorkLog {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      created_at: row.get("created_at")?,
      ended_at: row.get("ended_at")?,
      id: row.get("id")?,
      note: row.get("note")?,
      source: row.get("source")?,
      started_at: row.get("started_at")?,
      task_id: row.get("task_id")?,
      updated_at: row.get("updated_at")?,
    })
  }
}
