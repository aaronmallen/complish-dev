use diesel::prelude::*;
use eyre::Result;

use super::{List, entities::RelatedTask};
use crate::{models::schema::list_tasks, store::with_connection};

impl List {
  pub fn add_task(&mut self, task_id: impl Into<String>) -> Result<()> {
    with_connection(|connection| {
      let related_task = RelatedTask::new(self.id(), task_id);
      diesel::insert_into(list_tasks::table)
        .values(&related_task)
        .execute(connection)?;

      Ok(())
    })
  }

  pub fn remove_task(&mut self, task_id: impl Into<String>) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(list_tasks::table.filter(list_tasks::task_id.eq(task_id.into())))
        .execute(connection)?;

      Ok(())
    })
  }
}
