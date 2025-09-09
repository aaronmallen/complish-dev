use chrono::Utc;
use eyre::{Result, eyre};

use super::{
  Project,
  types::{Resolution, WorkflowStatus},
};

impl Project {
  pub fn cancel(&mut self) -> Result<()> {
    if self.workflow_status() == WorkflowStatus::Done {
      return Err(eyre!("Cannot cancel a project that is already done"));
    }

    self.set_workflow_status(WorkflowStatus::Done);
    self.set_resolution(Some(Resolution::Canceled));
    self.save()
  }

  pub fn complete(&mut self) -> Result<()> {
    if self.workflow_status() == WorkflowStatus::Done {
      return Err(eyre!("Cannot complete a project that is already done"));
    }

    self.set_workflow_status(WorkflowStatus::Done);
    self.set_resolution(Some(Resolution::Completed));
    self.set_completed_at(Some(Utc::now().naive_utc()));
    self.save()
  }

  pub fn reopen(&mut self) -> Result<()> {
    if self.workflow_status() != WorkflowStatus::Done {
      return Err(eyre!("Cannot reopen a project that is not done"));
    }

    self.set_workflow_status(WorkflowStatus::Todo);
    self.set_completed_at(None);
    self.set_resolution(None);
    self.save()
  }

  pub fn start(&mut self) -> Result<()> {
    if self.workflow_status() == WorkflowStatus::Done {
      return Err(eyre!("Cannot start a project that is already done"));
    }

    self.set_workflow_status(WorkflowStatus::InProgress);
    self.save()
  }

  pub fn plan(&mut self) -> Result<()> {
    if self.workflow_status() == WorkflowStatus::Done {
      return Err(eyre!("Cannot plan a project that is already done"));
    }

    self.set_workflow_status(WorkflowStatus::Planned);
    self.save()
  }

  pub fn pause(&mut self) -> Result<()> {
    if self.workflow_status() != WorkflowStatus::InProgress {
      return Err(eyre!("Cannot pause a project that is not in progress"));
    }

    self.set_workflow_status(WorkflowStatus::Todo);
    self.save()
  }
}
