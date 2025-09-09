use diesel::prelude::*;
use eyre::Result;

use super::{Project, entities::RelatedTask};
use crate::{models::schema::project_tasks, store::with_connection};

impl Project {
  pub fn add_task(&mut self, task_id: impl Into<String>) -> Result<()> {
    with_connection(|connection| {
      let related_task = RelatedTask::new(self.id(), task_id);
      diesel::insert_into(project_tasks::table)
        .values(&related_task)
        .execute(connection)?;

      Ok(())
    })
  }

  pub fn remove_task(&mut self, task_id: impl Into<String>) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(project_tasks::table.filter(project_tasks::task_id.eq(task_id.into())))
        .execute(connection)?;

      Ok(())
    })
  }
}
