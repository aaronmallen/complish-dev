use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

use super::Kind;

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize, Setters)]
pub struct Relationship {
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[getset(get = "pub", set = "pub")]
  kind: Kind,
  #[get = "pub"]
  source_id: String,
  #[get = "pub"]
  target_id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: DateTime<Utc>,
}

impl crate::task::Relationship {
  pub fn new(source_id: impl Into<String>, target_id: impl Into<String>, kind: Kind) -> Self {
    let now = Utc::now();

    Self {
      created_at: now,
      kind,
      source_id: source_id.into(),
      target_id: target_id.into(),
      updated_at: now,
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 5] {
    [
      &self.source_id,
      &self.target_id,
      &self.kind,
      &self.created_at,
      &self.updated_at,
    ]
  }
}

impl TryFrom<&Row<'_>> for crate::task::Relationship {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      created_at: row.get("created_at")?,
      kind: row.get("kind")?,
      source_id: row.get("source_id")?,
      target_id: row.get("target_id")?,
      updated_at: row.get("updated_at")?,
    })
  }
}
