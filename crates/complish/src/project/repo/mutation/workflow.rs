use eyre::Result;

use super::constructor::*;
use crate::{Project, ProjectResolution, ProjectWorkflowStatus};

impl<'a> Repo<'a> {
  pub fn cancel(&self, mut project: Project) -> Result<Project> {
    project.set_workflow_status(ProjectWorkflowStatus::Done);
    project.set_resolution(Some(ProjectResolution::Canceled));
    self.update(project)
  }

  pub fn complete(&self, mut project: Project) -> Result<Project> {
    project.set_workflow_status(ProjectWorkflowStatus::Done);
    project.set_resolution(Some(ProjectResolution::Completed));
    self.update(project)
  }

  pub fn mark_as_in_progress(&self, mut project: Project) -> Result<Project> {
    project.set_workflow_status(ProjectWorkflowStatus::InProgress);
    self.update(project)
  }

  pub fn mark_as_planned(&self, mut project: Project) -> Result<Project> {
    project.set_workflow_status(ProjectWorkflowStatus::Planned);
    self.update(project)
  }
}
