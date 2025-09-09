use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{Getters, MutGetters, Setters};

use super::Task;
use crate::{models::schema::task_notes, types::Metadata};

#[derive(
  Associations,
  Clone,
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
#[diesel(table_name = task_notes)]
#[diesel(belongs_to(Task, foreign_key = task_id))]
pub struct Note {
  #[getset(get = "pub", set = "pub")]
  content: String,
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", get_mut = "pub")]
  metadata: Metadata,
  #[get = "pub"]
  task_id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl Note {
  pub fn new(task_id: impl Into<String>, content: impl Into<String>) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      content: content.into(),
      created_at: now,
      id: cuid2::create_id(),
      metadata: Metadata::new(),
      task_id: task_id.into(),
      updated_at: now,
    }
  }
}
