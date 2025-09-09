use diesel::prelude::*;
use eyre::Result;

use super::Sprint;
use crate::{
  models::{
    Task,
    schema::{sprint_tasks, tasks},
  },
  store::with_connection,
};

impl Sprint {
  pub fn tasks(&self) -> Result<Vec<Task>> {
    with_connection(|connection| {
      Ok(
        sprint_tasks::table
          .inner_join(tasks::table.on(sprint_tasks::task_id.eq(tasks::id)))
          .filter(sprint_tasks::sprint_id.eq(self.id()))
          .select(Task::as_select())
          .load(connection)?,
      )
    })
  }
}
