use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{Getters, MutGetters, Setters};

use crate::{models::schema::sprints, types::Metadata};

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
#[diesel(table_name = sprints)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Sprint {
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[getset(get = "pub", set = "pub")]
  ended_at: NaiveDateTime,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", get_mut = "pub")]
  metadata: Metadata,
  #[getset(get = "pub", set = "pub")]
  started_at: NaiveDateTime,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl Default for Sprint {
  fn default() -> Self {
    Self::new()
  }
}

impl Sprint {
  pub fn new() -> Self {
    let now = Utc::now().naive_utc();
    let ended_at = now + Duration::hours(24);

    Self {
      created_at: now,
      ended_at,
      id: cuid2::create_id(),
      metadata: Metadata::new(),
      started_at: now,
      updated_at: now,
    }
  }
}
