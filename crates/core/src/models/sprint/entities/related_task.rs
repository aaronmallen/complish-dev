use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{Getters, Setters};

use super::Sprint;
use crate::models::{Task, schema::sprint_tasks};

#[derive(
  Associations,
  Clone,
  Debug,
  Getters,
  Identifiable,
  Insertable,
  PartialEq,
  Queryable,
  Selectable,
  Setters,
)]
#[diesel(table_name = sprint_tasks)]
#[diesel(primary_key(sprint_id, task_id))]
#[diesel(belongs_to(Sprint, foreign_key = sprint_id))]
#[diesel(belongs_to(Task, foreign_key = task_id))]
pub struct RelatedTask {
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[get = "pub"]
  sprint_id: String,
  #[get = "pub"]
  task_id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl RelatedTask {
  pub fn new(sprint_id: impl Into<String>, task_id: impl Into<String>) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      created_at: now,
      sprint_id: sprint_id.into(),
      task_id: task_id.into(),
      updated_at: now,
    }
  }
}
