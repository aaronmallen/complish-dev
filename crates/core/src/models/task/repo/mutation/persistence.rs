use chrono::Utc;
use diesel::prelude::*;
use eyre::Result;

use super::entities::Task;
use crate::{models::schema::tasks, store::with_connection};

impl Task {
  pub fn create(title: impl Into<String>) -> Result<Self> {
    let task = Task::new(title);

    with_connection(|connection| {
      diesel::insert_into(tasks::table)
        .values(&task)
        .execute(connection)?;

      Ok(())
    })?;

    Self::find(task.id())
  }

  pub fn delete(&self) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(tasks::table.find(self.id())).execute(connection)?;
      Ok(())
    })
  }

  pub fn save(&mut self) -> Result<()> {
    with_connection(|connection| {
      diesel::insert_into(tasks::table)
        .values(&*self)
        .on_conflict(tasks::id)
        .do_update()
        .set((
          tasks::completed_at.eq(self.completed_at()),
          tasks::description.eq(self.description()),
          tasks::due_at.eq(self.due_at()),
          tasks::estimation.eq(self.estimation()),
          tasks::external_id.eq(self.external_id()),
          tasks::metadata.eq(self.metadata()),
          tasks::priority.eq(self.priority()),
          tasks::resolution.eq(self.resolution()),
          tasks::title.eq(self.title()),
          tasks::updated_at.eq(Utc::now().naive_utc()),
          tasks::workflow_status.eq(self.workflow_status()),
        ))
        .execute(connection)?;

      Ok(())
    })
  }
}
