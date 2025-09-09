use diesel::prelude::*;
use eyre::Result;

use super::Tag;
use crate::{models::schema::tags, store::with_connection};

impl Tag {
  pub fn find(id: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        tags::table
          .find(id.into())
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }

  pub fn find_by_label(label: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        tags::table
          .filter(tags::label.eq(label.into()))
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }
}
