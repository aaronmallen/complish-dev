use diesel::prelude::*;
use eyre::Result;

use super::entities::Task;
use crate::{models::schema::tasks, store::with_connection};

impl Task {
  pub fn find(id: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        tasks::table
          .find(id.into())
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }

  pub fn find_by_sequence_id(sequence_id: i32) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        tasks::table
          .filter(tasks::sequence_id.eq(sequence_id))
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }
}
