use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{Getters, MutGetters, Setters};

use crate::{models::schema::tags, types::Metadata};

#[derive(
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
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", set = "pub")]
  label: String,
  #[getset(get = "pub", get_mut = "pub")]
  metadata: Metadata,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl Tag {
  pub fn new(label: impl Into<String>) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      created_at: now,
      id: cuid2::create_id(),
      label: label.into(),
      metadata: Metadata::new(),
      updated_at: now,
    }
  }
}
