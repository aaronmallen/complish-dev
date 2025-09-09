use chrono::Utc;
use diesel::prelude::*;
use eyre::{Result, eyre};

use super::{
  entities::{Relationship, Task, WorkLog},
  types::{RelationshipKind, Resolution, WorkflowStatus},
};
use crate::{
  models::schema::{task_relationships, task_work_logs, tasks},
  store::with_connection,
};

impl Task {
  pub fn cancel(&mut self) -> Result<()> {
    if self.workflow_status() == &WorkflowStatus::Done {
      return Err(eyre!("Cannot cancel a task that is already done"));
    }

    self.stop_work()?;
    self.set_workflow_status(WorkflowStatus::Done);
    self.set_resolution(Some(Resolution::Canceled));
    self.save()?;
    self.unblock_dependencies()
  }

  pub fn complete(&mut self) -> Result<()> {
    if self.workflow_status() == &WorkflowStatus::Done {
      return Err(eyre!("Cannot complete a task that is already done"));
    }

    self.stop_work()?;
    self.set_workflow_status(WorkflowStatus::Done);
    self.set_resolution(Some(Resolution::Completed));
    self.set_completed_at(Some(Utc::now().naive_utc()));
    self.save()?;
    self.unblock_dependencies()
  }

  pub fn delegate(&mut self) -> Result<()> {
    if self.workflow_status() == &WorkflowStatus::Done {
      return Err(eyre!("Cannot delegate a task that is already done"));
    }

    self.stop_work()?;
    self.set_workflow_status(WorkflowStatus::Done);
    self.set_resolution(Some(Resolution::Delegated));
    self.save()?;
    self.unblock_dependencies()
  }

  pub fn reopen(&mut self) -> Result<()> {
    if self.workflow_status() != &WorkflowStatus::Done {
      return Err(eyre!("Cannot reopen a task that is not done"));
    }

    self.set_workflow_status(WorkflowStatus::Todo);
    self.set_completed_at(None);
    self.set_resolution(None);
    self.save()
  }

  pub fn start_work(&mut self) -> Result<()> {
    if self.workflow_status() == &WorkflowStatus::Done {
      return Err(eyre!("Cannot start work on a task that is already done"));
    }

    if self.workflow_status() == &WorkflowStatus::Blocked {
      return Err(eyre!("Cannot start work on a task that is blocked"));
    }

    self.stop_work()?;

    let work_log = WorkLog::new(self.id());
    with_connection(|connection| {
      diesel::insert_into(task_work_logs::table)
        .values(&work_log)
        .execute(connection)
        .map_err(|e| eyre!("Failed to create work log: {}", e))?;
      Ok(())
    })?;

    self.set_workflow_status(WorkflowStatus::InProgress);
    self.save()
  }

  pub fn stop_work(&mut self) -> Result<()> {
    with_connection(|connection| {
      diesel::update(task_work_logs::table)
        .filter(
          task_work_logs::task_id
            .eq(self.id())
            .and(task_work_logs::ended_at.is_null()),
        )
        .set((
          task_work_logs::ended_at.eq(Some(Utc::now().naive_utc())),
          task_work_logs::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(connection)?;

      Ok(())
    })?;

    self.save()
  }

  pub fn unblock(&mut self) -> Result<()> {
    if self.workflow_status() != &WorkflowStatus::Blocked {
      return Err(eyre!("Cannot unblock a task that is not blocked"));
    }

    if self.workflow_status() == &WorkflowStatus::Done {
      return Err(eyre!("Cannot unblock a task that is already done"));
    }

    self.set_workflow_status(WorkflowStatus::Todo);
    self.save()
  }

  fn unblock_dependencies(&self) -> Result<()> {
    with_connection(|connection| {
      let blocking = task_relationships::table
        .filter(
          task_relationships::source_id
            .eq(self.id())
            .and(task_relationships::kind.eq(RelationshipKind::Blocks)),
        )
        .select(Relationship::as_select())
        .load(connection)?;

      for blocker in blocking {
        let other_blockers = task_relationships::table
          .inner_join(tasks::table.on(task_relationships::source_id.eq(tasks::id)))
          .filter(
            task_relationships::target_id
              .eq(blocker.target_id())
              .and(task_relationships::kind.eq(RelationshipKind::Blocks))
              .and(task_relationships::source_id.ne(self.id()))
              .and(tasks::workflow_status.ne(WorkflowStatus::Done)),
          )
          .select(task_relationships::source_id)
          .first::<String>(connection)
          .optional()?;

        if other_blockers.is_none() {
          diesel::update(
            tasks::table.filter(
              tasks::workflow_status
                .eq(WorkflowStatus::Blocked)
                .and(tasks::id.eq(blocker.target_id())),
            ),
          )
          .set((
            tasks::workflow_status.eq(WorkflowStatus::Todo),
            tasks::updated_at.eq(Utc::now().naive_utc()),
          ))
          .execute(connection)?;
        }
      }

      Ok(())
    })
  }
}
