use std::convert::TryFrom;

use chrono::{DateTime, Utc};
use getset::{CloneGetters, Getters, MutGetters, Setters};
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

use super::{Resolution, Update, WorkflowStatus};
use crate::Tag;

#[derive(
  Clone, CloneGetters, Debug, Deserialize, Eq, Getters, MutGetters, PartialEq, Serialize, Setters,
)]
pub struct Project {
  #[getset(get = "pub", set = "pub")]
  completed_at: Option<DateTime<Utc>>,
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[getset(get = "pub", set = "pub")]
  description: Option<String>,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", set = "pub")]
  key: String,
  #[getset(get = "pub", set = "pub")]
  name: String,
  #[getset(get_clone = "pub", set = "pub")]
  resolution: Option<Resolution>,
  #[getset(get = "pub", get_mut = "pub")]
  pub(crate) tags: Vec<Tag>,
  #[getset(get = "pub", get_mut = "pub")]
  pub(crate) updates: Vec<Update>,
  #[getset(get_clone = "pub", set = "pub")]
  updated_at: DateTime<Utc>,
  #[getset(get_clone = "pub", set = "pub")]
  workflow_status: WorkflowStatus,
}

impl Project {
  pub fn new(name: impl Into<String>) -> Self {
    let name = name.into();
    let now = Utc::now();

    let key = if name.len() >= 3 {
      name[0..3].to_uppercase()
    } else {
      name.to_uppercase()
    };

    Self {
      completed_at: None,
      created_at: now,
      description: None,
      id: cuid2::create_id(),
      key,
      name,
      resolution: None,
      tags: Vec::new(),
      updates: Vec::new(),
      updated_at: now,
      workflow_status: WorkflowStatus::default(),
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 9] {
    [
      &self.id,
      &self.name,
      &self.key,
      &self.description,
      &self.workflow_status,
      &self.resolution,
      &self.completed_at,
      &self.created_at,
      &self.updated_at,
    ]
  }

  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  pub fn with_key(mut self, key: impl Into<String>) -> Self {
    self.key = key.into();
    self
  }
}

impl TryFrom<&Row<'_>> for Project {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      completed_at: row.get("completed_at")?,
      created_at: row.get("created_at")?,
      description: row.get("description")?,
      id: row.get("id")?,
      key: row.get("key")?,
      name: row.get("name")?,
      resolution: row.get("resolution")?,
      tags: Vec::new(),
      updates: Vec::new(),
      updated_at: row.get("updated_at")?,
      workflow_status: row.get("workflow_status")?,
    })
  }
}
