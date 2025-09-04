use chrono::{DateTime, Duration, Utc};
use getset::{Getters, MutGetters, Setters};
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

use crate::Task;

#[derive(Clone, Debug, Deserialize, Eq, Getters, MutGetters, PartialEq, Serialize, Setters)]
pub struct Sprint {
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[getset(get = "pub", set = "pub")]
  ended_at: DateTime<Utc>,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", set = "pub")]
  started_at: DateTime<Utc>,
  #[getset(get = "pub", get_mut = "pub")]
  pub(crate) tasks: Vec<Task>,
  #[getset(get = "pub", set = "pub")]
  updated_at: DateTime<Utc>,
}

impl Sprint {
  pub fn new(started_at: DateTime<Utc>) -> Self {
    let now = Utc::now();
    let ended_at = started_at + Duration::hours(24);

    Self {
      created_at: now,
      ended_at,
      id: cuid2::create_id(),
      started_at,
      tasks: Vec::new(),
      updated_at: now,
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 5] {
    [
      &self.id,
      &self.started_at,
      &self.ended_at,
      &self.created_at,
      &self.updated_at,
    ]
  }
}

impl TryFrom<&Row<'_>> for Sprint {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      created_at: row.get("created_at")?,
      ended_at: row.get("ended_at")?,
      id: row.get("id")?,
      started_at: row.get("started_at")?,
      tasks: Vec::new(),
      updated_at: row.get("updated_at")?,
    })
  }
}
