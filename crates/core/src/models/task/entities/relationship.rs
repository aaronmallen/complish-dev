use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use getset::{Getters, MutGetters, Setters};

use super::types::RelationshipKind;
use crate::{models::schema::task_relationships, types::Metadata};

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
#[diesel(table_name = task_relationships)]
#[diesel(primary_key(source_id, target_id, kind))]
pub struct Relationship {
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[get = "pub"]
  kind: RelationshipKind,
  #[getset(get = "pub", get_mut = "pub")]
  metadata: Metadata,
  #[get = "pub"]
  source_id: String,
  #[get = "pub"]
  target_id: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl Relationship {
  pub fn new(
    kind: RelationshipKind,
    source_id: impl Into<String>,
    target_id: impl Into<String>,
  ) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      created_at: now,
      kind,
      metadata: Metadata::new(),
      source_id: source_id.into(),
      target_id: target_id.into(),
      updated_at: now,
    }
  }
}
