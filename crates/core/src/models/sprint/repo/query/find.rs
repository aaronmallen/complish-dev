use chrono::Utc;
use diesel::prelude::*;
use eyre::Result;

use super::Sprint;
use crate::{models::schema::sprints, store::with_connection};

impl Sprint {
  pub fn current() -> Result<Self> {
    with_connection(|connection| {
      let now = Utc::now().naive_utc();
      Ok(
        sprints::table
          .filter(sprints::started_at.le(now).and(sprints::ended_at.ge(now)))
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }

  pub fn find(id: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        sprints::table
          .find(id.into())
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }
}
