use diesel::prelude::*;
use eyre::Result;

use super::entities::{Note, Relationship, Task, WorkLog};
use crate::{
  Tag,
  models::schema::{tags, task_relationships, task_tags},
  store::with_connection,
};

impl Task {
  pub fn notes(&self) -> Result<Vec<Note>> {
    with_connection(|connection| {
      Ok(
        Note::belonging_to(self)
          .select(Note::as_select())
          .load(connection)?,
      )
    })
  }

  pub fn relationships(&self) -> Result<Vec<Relationship>> {
    with_connection(|connection| {
      Ok(
        task_relationships::table
          .filter(task_relationships::source_id.eq(self.id()))
          .select(Relationship::as_select())
          .load(connection)?,
      )
    })
  }

  pub fn tags(&self) -> Result<Vec<Tag>> {
    with_connection(|connection| {
      Ok(
        tags::table
          .inner_join(task_tags::table.on(tags::id.eq(task_tags::tag_id)))
          .filter(task_tags::task_id.eq(self.id()))
          .select(Tag::as_select())
          .load(connection)?,
      )
    })
  }

  pub fn work_logs(&self) -> Result<Vec<WorkLog>> {
    with_connection(|connection| {
      Ok(
        WorkLog::belonging_to(self)
          .select(WorkLog::as_select())
          .load(connection)?,
      )
    })
  }
}
