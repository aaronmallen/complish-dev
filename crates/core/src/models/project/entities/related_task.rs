use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{Getters, Setters};

use super::Project;
use crate::models::{Task, schema::project_tasks};

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
#[diesel(table_name = project_tasks)]
#[diesel(primary_key(project_id, task_id))]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(belongs_to(Task, foreign_key = task_id))]
pub struct RelatedTask {
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[get = "pub"]
  project_id: String,
  #[get = "pub"]
  task_id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl RelatedTask {
  pub fn new(project_id: impl Into<String>, task_id: impl Into<String>) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      created_at: now,
      project_id: project_id.into(),
      task_id: task_id.into(),
      updated_at: now,
    }
  }
}
