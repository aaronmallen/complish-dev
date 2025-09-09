use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{Getters, MutGetters, Setters};

use crate::{models::schema::journal_entries, types::Metadata};

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
#[diesel(table_name = journal_entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
  #[getset(get = "pub", set = "pub")]
  content: String,
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", get_mut = "pub")]
  metadata: Metadata,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl Entry {
  pub fn new(content: impl Into<String>) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      content: content.into(),
      created_at: now,
      id: cuid2::create_id(),
      metadata: Metadata::new(),
      updated_at: now,
    }
  }
}
