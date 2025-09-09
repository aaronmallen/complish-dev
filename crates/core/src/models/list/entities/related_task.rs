use chrono::{NaiveDateTime, Utc};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use getset::{Getters, Setters};

use super::List;
use crate::models::{Task, schema::list_tasks};

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
#[diesel(table_name = list_tasks)]
#[diesel(primary_key(list_id, task_id))]
#[diesel(belongs_to(List, foreign_key = list_id))]
#[diesel(belongs_to(Task, foreign_key = task_id))]
pub struct RelatedTask {
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[get = "pub"]
  list_id: String,
  #[get = "pub"]
  task_id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl RelatedTask {
  pub fn new(list_id: impl Into<String>, task_id: impl Into<String>) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      created_at: now,
      list_id: list_id.into(),
      task_id: task_id.into(),
      updated_at: now,
    }
  }
}
