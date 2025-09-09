use diesel::prelude::*;
use eyre::{Result, eyre};

use super::{
  entities::{Relationship, Task},
  types::{RelationshipKind, WorkflowStatus},
};
use crate::{models::schema::task_relationships, store::with_connection};

impl Task {
  pub fn block(&mut self, blocker_id: impl Into<String>) -> Result<()> {
    if self.workflow_status() == &WorkflowStatus::Done {
      return Err(eyre!("Cannot block a task that is already done"));
    }

    let blocker_id = blocker_id.into();
    let blocked = Relationship::new(RelationshipKind::Depends, self.id(), &blocker_id);
    let blocker = Relationship::new(RelationshipKind::Blocks, &blocker_id, self.id());

    with_connection(|connection| {
      diesel::insert_into(task_relationships::table)
        .values(&[blocked, blocker])
        .execute(connection)
        .map_err(|e| eyre!("Failed to create blocking relationship: {}", e))?;

      Ok(())
    })?;

    self.set_workflow_status(WorkflowStatus::Blocked);
    self.save()
  }

  pub fn link(&mut self, target_id: impl Into<String>) -> Result<()> {
    let target_id = target_id.into();
    let source = Relationship::new(RelationshipKind::Relates, self.id(), &target_id);
    let target = Relationship::new(RelationshipKind::Relates, &target_id, self.id());

    with_connection(|connection| {
      diesel::insert_into(task_relationships::table)
        .values(&[source, target])
        .execute(connection)
        .map_err(|e| eyre!("Failed to create relationship: {}", e))?;

      Ok(())
    })?;

    self.save()
  }

  pub fn remove_relation(
    &mut self,
    kind: RelationshipKind,
    target_id: impl Into<String>,
  ) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(
        task_relationships::table.filter(
          task_relationships::kind
            .eq(kind)
            .and(task_relationships::source_id.eq(self.id()))
            .and(task_relationships::target_id.eq(target_id.into())),
        ),
      )
      .execute(connection)?;

      Ok(())
    })?;

    self.save()
  }
}
