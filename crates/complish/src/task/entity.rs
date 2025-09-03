use std::convert::TryFrom;

use chrono::{DateTime, Utc};
use getset::{CloneGetters, Getters, MutGetters, Setters};
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

use super::{Estimation, Note, Priority, Relationship, Resolution, WorkLog, WorkflowStatus};
use crate::Tag;

#[derive(
  Clone, CloneGetters, Debug, Deserialize, Eq, Getters, MutGetters, PartialEq, Serialize, Setters,
)]
pub struct Task {
  #[getset(get_clone = "pub", set = "pub")]
  completed_at: Option<DateTime<Utc>>,
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[getset(get_clone = "pub", set = "pub")]
  description: Option<String>,
  #[getset(get_clone = "pub", set = "pub")]
  due_at: Option<DateTime<Utc>>,
  #[getset(get_clone = "pub", set = "pub")]
  estimation: Option<Estimation>,
  #[getset(get_clone = "pub", set = "pub")]
  external_id: Option<String>,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", get_mut = "pub")]
  pub(crate) notes: Vec<Note>,
  #[getset(get = "pub", set = "pub")]
  priority: Priority,
  #[getset(get_clone = "pub", get_mut = "pub")]
  project_id: Option<String>,
  #[getset(get = "pub", get_mut = "pub")]
  pub(crate) relationships: Vec<Relationship>,
  #[getset(get_clone = "pub", set = "pub")]
  resolution: Option<Resolution>,
  #[get = "pub"]
  sequence_id: u32,
  #[getset(get = "pub", get_mut = "pub")]
  pub(crate) tags: Vec<Tag>,
  #[getset(get = "pub", set = "pub")]
  title: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: DateTime<Utc>,
  #[getset(get = "pub", get_mut = "pub")]
  pub(crate) work_logs: Vec<WorkLog>,
  #[getset(get = "pub", set = "pub")]
  workflow_status: WorkflowStatus,
}

impl Task {
  pub fn new(title: impl Into<String>) -> Self {
    let now = Utc::now();

    Self {
      completed_at: None,
      created_at: now,
      description: None,
      due_at: None,
      estimation: None,
      external_id: None,
      id: cuid2::create_id(),
      notes: Vec::new(),
      priority: Priority::default(),
      project_id: None,
      relationships: Vec::new(),
      resolution: None,
      sequence_id: 0,
      tags: Vec::new(),
      title: title.into(),
      updated_at: now,
      work_logs: Vec::new(),
      workflow_status: WorkflowStatus::default(),
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 13] {
    [
      &self.id,
      &self.external_id,
      &self.project_id,
      &self.title,
      &self.description,
      &self.priority,
      &self.workflow_status,
      &self.estimation,
      &self.resolution,
      &self.due_at,
      &self.completed_at,
      &self.created_at,
      &self.updated_at,
    ]
  }
}

impl TryFrom<&Row<'_>> for Task {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      completed_at: row.get("completed_at")?,
      created_at: row.get("created_at")?,
      description: row.get("description")?,
      due_at: row.get("due_at")?,
      estimation: row.get("estimation")?,
      external_id: row.get("external_id")?,
      id: row.get("id")?,
      notes: Vec::new(),
      priority: row.get("priority")?,
      project_id: row.get("project_id")?,
      relationships: Vec::new(),
      resolution: row.get("resolution")?,
      sequence_id: row.get("sequence_id")?,
      tags: Vec::new(),
      title: row.get("title")?,
      updated_at: row.get("updated_at")?,
      work_logs: Vec::new(),
      workflow_status: row.get("workflow_status")?,
    })
  }
}
