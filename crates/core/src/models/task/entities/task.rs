use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{CloneGetters, Getters, MutGetters, Setters};

use super::types::{Estimation, Priority, Resolution, WorkflowStatus};
use crate::{models::schema::tasks, types::Metadata};

#[derive(
  Clone,
  CloneGetters,
  Debug,
  Eq,
  Getters,
  Identifiable,
  Insertable,
  MutGetters,
  PartialEq,
  Queryable,
  Selectable,
  Setters,
)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
  #[getset(get_clone = "pub", set = "pub")]
  completed_at: Option<NaiveDateTime>,
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[getset(get_clone = "pub", set = "pub")]
  description: Option<String>,
  #[getset(get_clone = "pub", set = "pub")]
  due_at: Option<NaiveDateTime>,
  #[getset(get_clone = "pub", set = "pub")]
  estimation: Option<Estimation>,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", get_mut = "pub")]
  metadata: Metadata,
  #[getset(get = "pub", set = "pub")]
  priority: Priority,
  #[getset(get_clone = "pub", set = "pub")]
  resolution: Option<Resolution>,
  #[get_clone = "pub"]
  sequence_id: Option<i32>,
  #[getset(get = "pub", set = "pub")]
  title: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
  #[getset(get = "pub", set = "pub")]
  workflow_status: WorkflowStatus,
}

impl Task {
  pub fn new(title: impl Into<String>) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      completed_at: None,
      created_at: now,
      description: None,
      due_at: None,
      estimation: None,
      id: cuid2::create_id(),
      metadata: Metadata::new(),
      priority: Priority::default(),
      resolution: None,
      sequence_id: None,
      title: title.into(),
      updated_at: now,
      workflow_status: WorkflowStatus::default(),
    }
  }

  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  pub fn with_due_date(mut self, due_at: NaiveDateTime) -> Self {
    self.due_at = Some(due_at);
    self
  }

  pub fn with_estimation(mut self, estimation: Estimation) -> Self {
    self.estimation = Some(estimation);
    self
  }

  pub fn with_priority(mut self, priority: Priority) -> Self {
    self.priority = priority;
    self
  }
}
