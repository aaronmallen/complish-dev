use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{CloneGetters, Getters, MutGetters, Setters};

use super::{Project, UpdateStatus};
use crate::{models::schema::project_updates, types::Metadata};

#[derive(
  Associations,
  Clone,
  CloneGetters,
  Debug,
  Getters,
  Identifiable,
  Insertable,
  MutGetters,
  PartialEq,
  Queryable,
  Selectable,
  Setters,
)]
#[diesel(table_name = project_updates)]
#[diesel(belongs_to(Project, foreign_key = project_id))]
pub struct Update {
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[getset(get_clone = "pub", set = "pub")]
  description: Option<String>,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", get_mut = "pub")]
  metadata: Metadata,
  #[get = "pub"]
  project_id: String,
  #[getset(get = "pub", set = "pub")]
  status: UpdateStatus,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl Update {
  pub fn new(project_id: impl Into<String>, status: UpdateStatus) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      created_at: now,
      description: None,
      id: cuid2::create_id(),
      metadata: Metadata::new(),
      project_id: project_id.into(),
      status,
      updated_at: now,
    }
  }

  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }
}
