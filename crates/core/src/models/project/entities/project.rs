use std::path::PathBuf;

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{CloneGetters, Getters, MutGetters, Setters};

use super::entities::{Resolution, WorkflowStatus};
use crate::{
  models::schema::projects,
  types::{JsonVec, Metadata},
};

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
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
  #[getset(get_clone = "pub", set = "pub")]
  completed_at: Option<NaiveDateTime>,
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[getset(get = "pub", get_mut = "pub")]
  directories: JsonVec<PathBuf>,
  #[getset(get_clone = "pub", set = "pub")]
  description: Option<String>,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", get_mut = "pub")]
  metadata: Metadata,
  #[getset(get = "pub", set = "pub")]
  key: String,
  #[getset(get = "pub", set = "pub")]
  name: String,
  #[getset(get_clone = "pub", set = "pub")]
  resolution: Option<Resolution>,
  #[getset(get_clone = "pub", set = "pub")]
  updated_at: NaiveDateTime,
  #[getset(get_clone = "pub", set = "pub")]
  workflow_status: WorkflowStatus,
}

impl Project {
  pub fn new(name: impl Into<String>) -> Project {
    let name = name.into();
    let now = Utc::now().naive_utc();

    let key = if name.len() >= 3 {
      name[0..3].to_uppercase()
    } else {
      name.to_uppercase()
    };

    Project {
      completed_at: None,
      created_at: now,
      directories: JsonVec::new(),
      description: None,
      id: cuid2::create_id(),
      metadata: Metadata::new(),
      key,
      name,
      resolution: None,
      updated_at: now,
      workflow_status: WorkflowStatus::default(),
    }
  }
}
