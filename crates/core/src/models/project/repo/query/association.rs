use diesel::prelude::*;
use eyre::Result;

use super::{Project, Update};
use crate::{
  models::{
    Task,
    schema::{project_tasks, project_updates, tasks},
  },
  store::with_connection,
};

impl Project {
  pub fn tasks(&self) -> Result<Vec<Task>> {
    with_connection(|connection| {
      Ok(
        tasks::table
          .inner_join(project_tasks::table.on(tasks::id.eq(project_tasks::task_id)))
          .filter(project_tasks::project_id.eq(self.id()))
          .select(Task::as_select())
          .load(connection)?,
      )
    })
  }

  pub fn updates(&self) -> Result<Vec<Update>> {
    with_connection(|connection| {
      Ok(
        project_updates::table
          .filter(project_updates::project_id.eq(self.id()))
          .select(Update::as_select())
          .load(connection)?,
      )
    })
  }
}
