use diesel::prelude::*;
use eyre::Result;

use super::{Sprint, entities::RelatedTask};
use crate::{
  models::schema::sprint_tasks,
  store::with_connection,
};

impl Sprint {
  pub fn add_task(&mut self, task_id: impl Into<String>) -> Result<()> {
    let related_task = RelatedTask::new(self.id(), task_id);

    with_connection(|connection| {
      diesel::insert_into(sprint_tasks::table)
        .values(&related_task)
        .execute(connection)?;

      Ok(())
    })?;

    self.save()
  }

  pub fn remove_task(&mut self, task_id: impl Into<String>) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(
        sprint_tasks::table.filter(
          sprint_tasks::task_id
            .eq(task_id.into())
            .and(sprint_tasks::sprint_id.eq(self.id())),
        ),
      )
      .execute(connection)?;
      Ok(())
    })?;

    self.save()
  }
}
