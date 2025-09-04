use eyre::{Result, eyre};

use super::constructor::*;
use crate::{Task, TaskResolution, TaskWorkflowStatus};

impl<'a> Repo<'a> {
  pub fn cancel(&self, mut task: Task) -> Result<Task> {
    if task.workflow_status() == &TaskWorkflowStatus::Done {
      return Err(eyre!("Cannot cancel a task that is already done"));
    }

    task.set_resolution(Some(TaskResolution::Canceled));
    task.set_workflow_status(TaskWorkflowStatus::Done);
    self.update(task)
  }

  pub fn complete(&self, mut task: Task) -> Result<Task> {
    if task.workflow_status() == &TaskWorkflowStatus::Done {
      return Err(eyre!("Cannot complete a task that is already done"));
    }

    task.set_resolution(Some(TaskResolution::Completed));
    task.set_workflow_status(TaskWorkflowStatus::Done);
    self.update(task)
  }

  pub fn delegate(&self, mut task: Task) -> Result<Task> {
    if task.workflow_status() == &TaskWorkflowStatus::Done {
      return Err(eyre!("Cannot delegate a task that is already done"));
    }

    task.set_resolution(Some(TaskResolution::Delegated));
    task.set_workflow_status(TaskWorkflowStatus::Done);
    self.update(task)
  }
}
