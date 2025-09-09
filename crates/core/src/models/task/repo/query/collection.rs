use diesel::prelude::*;
use eyre::Result;

use super::{entities::Task, types::WorkflowStatus};
use crate::{models::schema::tasks, store::with_connection};

impl Task {
  pub fn all() -> Result<Vec<Task>> {
    with_connection(|connection| Ok(tasks::table.select(Self::as_select()).load(connection)?))
  }

  pub fn all_active() -> Result<Vec<Task>> {
    with_connection(|connection| {
      Ok(
        tasks::table
          .filter(
            tasks::workflow_status.ne_all(vec![WorkflowStatus::Done, WorkflowStatus::Blocked]),
          )
          .select(Self::as_select())
          .load(connection)?,
      )
    })
  }
}
