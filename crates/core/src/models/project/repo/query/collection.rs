use diesel::prelude::*;
use eyre::Result;

use super::{Project, WorkflowStatus};
use crate::{models::schema::projects, store::with_connection};

impl Project {
  pub fn all() -> Result<Vec<Self>> {
    with_connection(|connection| Ok(projects::table.select(Self::as_select()).load(connection)?))
  }

  pub fn all_active() -> Result<Vec<Self>> {
    with_connection(|connection| {
      Ok(
        projects::table
          .filter(projects::workflow_status.ne(WorkflowStatus::Done))
          .select(Self::as_select())
          .load(connection)?,
      )
    })
  }
}
