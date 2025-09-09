use chrono::NaiveDateTime;
use diesel::prelude::*;
use getset::{CloneGetters, Getters, MutGetters, Setters};

use super::Task;
use crate::{models::schema::task_work_logs, types::Metadata};

#[derive(
  Associations,
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
#[diesel(table_name = task_work_logs)]
#[diesel(belongs_to(Task, foreign_key = task_id))]
pub struct WorkLog {
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[getset(get_clone = "pub", set = "pub")]
  ended_at: Option<NaiveDateTime>,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", get_mut = "pub")]
  metadata: Metadata,
  #[getset(get_clone = "pub", set = "pub")]
  note: Option<String>,
  #[getset(get_clone = "pub", set = "pub")]
  source: Option<String>,
  #[getset(get = "pub", set = "pub")]
  started_at: NaiveDateTime,
  #[get = "pub"]
  task_id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl WorkLog {
  pub fn new(task_id: impl Into<String>) -> Self {
    let now = chrono::Utc::now().naive_utc();

    Self {
      created_at: now,
      ended_at: None,
      id: cuid2::create_id(),
      metadata: Metadata::new(),
      note: None,
      source: None,
      started_at: now,
      task_id: task_id.into(),
      updated_at: now,
    }
  }

  pub fn with_end_date(mut self, ended_at: NaiveDateTime) -> Self {
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

  pub fn with_start_date(mut self, started_at: NaiveDateTime) -> Self {
    self.started_at = started_at;
    self
  }
}
