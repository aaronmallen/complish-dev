use std::path::PathBuf;

use chrono::{NaiveDateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use getset::{Getters, MutGetters, Setters};

use crate::{
  models::schema::lists,
  types::{JsonVec, Metadata},
};

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
  Setters,
  Selectable,
)]
#[diesel(table_name = lists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct List {
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[getset(get = "pub", get_mut = "pub")]
  directories: JsonVec<PathBuf>,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", get_mut = "pub")]
  metadata: Metadata,
  #[getset(get = "pub", set = "pub")]
  name: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl List {
  pub fn new(name: impl Into<String>) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      created_at: now,
      directories: JsonVec::new(),
      id: cuid2::create_id(),
      metadata: Metadata::new(),
      name: name.into(),
      updated_at: now,
    }
  }
}
