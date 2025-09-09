use diesel::prelude::*;
use eyre::Result;

use super::Project;
use crate::{models::schema::projects, store::with_connection};

impl Project {
  pub fn find(id: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        projects::table
          .find(id.into())
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }

  pub fn find_by_key(key: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        projects::table
          .filter(projects::key.eq(key.into()))
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }
}
