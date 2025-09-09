use diesel::prelude::*;
use eyre::Result;

use super::List;
use crate::{
  models::{
    Task,
    schema::{list_tasks, tasks},
  },
  store::with_connection,
};

impl List {
  pub fn tasks(&self) -> Result<Vec<Task>> {
    with_connection(|connection| {
      Ok(
        list_tasks::table
          .inner_join(tasks::table.on(list_tasks::task_id.eq(tasks::id)))
          .filter(list_tasks::list_id.eq(self.id()))
          .select(Task::as_select())
          .load(connection)?,
      )
    })
  }
}
