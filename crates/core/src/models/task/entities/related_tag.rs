use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{Getters, Setters};

use super::Task;
use crate::{Tag, models::schema::task_tags};

#[derive(
  Associations,
  Clone,
  Debug,
  Eq,
  Getters,
  Identifiable,
  Insertable,
  PartialEq,
  Queryable,
  Selectable,
  Setters,
)]
#[diesel(table_name = task_tags)]
#[diesel(primary_key(task_id, tag_id))]
#[diesel(belongs_to(Task, foreign_key = task_id))]
#[diesel(belongs_to(Tag, foreign_key = tag_id))]
pub struct RelatedTag {
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[get = "pub"]
  tag_id: String,
  #[get = "pub"]
  task_id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl RelatedTag {
  pub fn new(task_id: impl Into<String>, tag_id: impl Into<String>) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      created_at: now,
      tag_id: tag_id.into(),
      task_id: task_id.into(),
      updated_at: now,
    }
  }
}
