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

#[cfg(test)]
mod test {
  use super::*;
  use crate::macros::with_test_connection;

  mod create {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_creates_a_new_task() {
      with_test_connection!({
        let task = Task::create("a test task").unwrap();

        assert_eq!(task, Task::find(task.id()).unwrap());
      })
    }
  }

  mod delete {
    use super::*;

    #[test]
    fn it_deletes_the_task() {
      with_test_connection!({
        let mut task = Task::new("a test task");
        task.save().unwrap();
        task.delete().unwrap();
        assert!(Task::find(task.id()).is_err());
      })
    }
  }

  mod save {
    use pretty_assertions::assert_ne;

    use super::*;

    #[test]
    fn it_saves_the_task() {
      with_test_connection!({
        let mut task = Task::new("a test task");
        task.save().unwrap();

        assert_eq!(task.id(), Task::find(task.id()).unwrap().id());
      })
    }

    #[test]
    fn it_updates_the_task_if_it_already_exists() {
      with_test_connection!({
        let mut task = Task::new("a test task");
        task.save().unwrap();
        task.save().unwrap();

        assert_ne!(
          task.updated_at(),
          Task::find(task.id()).unwrap().updated_at()
        );
      })
    }
  }
}
