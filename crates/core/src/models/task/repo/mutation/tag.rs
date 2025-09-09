use diesel::prelude::*;
use eyre::Result;

use super::entities::{RelatedTag, Task};
use crate::{
  models::{Tag, schema::task_tags},
  store::with_connection,
};

impl Task {
  pub fn add_tag(&mut self, label: impl Into<String>) -> Result<()> {
    with_connection(|connection| {
      let tag = Tag::find_or_create(label)?;
      let related_tag = RelatedTag::new(self.id(), tag.id());

      diesel::insert_into(task_tags::table)
        .values(&related_tag)
        .execute(connection)?;

      Ok(())
    })
  }

  pub fn delete_tag(&mut self, label: impl Into<String>) -> Result<()> {
    with_connection(|connection| {
      let tag = Tag::find_by_label(label)?;
      diesel::delete(
        task_tags::table.filter(
          task_tags::task_id
            .eq(self.id())
            .and(task_tags::tag_id.eq(tag.id())),
        ),
      )
      .execute(connection)?;

      Ok(())
    })
  }
}
