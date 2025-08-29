pub(crate) mod estimation;
pub(crate) mod note;
pub(crate) mod priority;
pub(crate) mod relationship;
pub(crate) mod relationship_type;
pub(crate) mod resolution;
pub(crate) mod work_log;
pub(crate) mod workflow_status;

use chrono::{DateTime, Utc};
use cuid::cuid2;
use estimation::Estimation;
use eyre::{Result, eyre};
use getset::{CloneGetters, Getters};
use note::Note;
use priority::Priority;
use relationship::Relationship;
use resolution::Resolution;
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};
use work_log::WorkLog;
use workflow_status::WorkflowStatus;

use crate::{
  entities::{Project, Tag},
  types::JsonVec,
};

#[derive(Clone, CloneGetters, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct Task {
  #[get_clone = "pub"]
  completed_at: Option<DateTime<Utc>>,
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get_clone = "pub"]
  description: Option<String>,
  #[get_clone = "pub"]
  due_at: Option<DateTime<Utc>>,
  #[get_clone = "pub"]
  estimation: Option<Estimation>,
  #[get_clone = "pub"]
  external_id: Option<String>,
  #[get = "pub"]
  id: String,
  #[get = "pub"]
  notes: JsonVec<Note>,
  #[get = "pub"]
  priority: Priority,
  #[get_clone = "pub"]
  project_id: Option<String>,
  #[get = "pub"]
  pub(crate) relationships: Vec<Relationship>,
  #[get_clone = "pub"]
  resolution: Option<Resolution>,
  #[get_clone = "pub"]
  sequence_id: u32,
  #[get = "pub"]
  pub(crate) tags: Vec<Tag>,
  #[get = "pub"]
  title: String,
  #[get = "pub"]
  updated_at: DateTime<Utc>,
  #[get = "pub"]
  work_logs: JsonVec<WorkLog>,
  #[get = "pub"]
  workflow_status: WorkflowStatus,
}

impl Task {
  pub fn new(title: impl Into<String>) -> Self {
    let now = Utc::now();

    Self {
      completed_at: None,
      created_at: now,
      description: None,
      due_at: None,
      estimation: None,
      external_id: None,
      id: cuid2(),
      notes: JsonVec::new(),
      priority: Priority::default(),
      project_id: None,
      relationships: Vec::new(),
      resolution: None,
      sequence_id: 0,
      tags: Vec::new(),
      title: title.into(),
      updated_at: now,
      work_logs: JsonVec::new(),
      workflow_status: WorkflowStatus::default(),
    }
  }

  pub fn add_note(&mut self, note: impl Into<String>) {
    self.notes.push(Note::new(note));
    self.touch();
  }

  pub fn block(&mut self) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot block a task that is already done"));
    }

    self.stop();
    self.workflow_status = WorkflowStatus::Blocked;
    self.touch();

    Ok(())
  }

  pub fn block_with_note(&mut self, note: impl Into<String>) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot block a task that is already done"));
    }

    self.add_note(note);
    self.block()
  }

  pub fn cancel(&mut self) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot cancel a task that is already done"));
    }

    self.stop();
    self.workflow_status = WorkflowStatus::Done;
    self.resolution = Some(Resolution::Canceled);
    self.touch();

    Ok(())
  }

  pub fn cancel_with_note(&mut self, note: impl Into<String>) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot cancel a task that is already done"));
    }

    self.add_note(note);
    self.cancel()
  }

  pub fn complete(&mut self) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot complete a task that is already done"));
    }

    self.stop();
    self.workflow_status = WorkflowStatus::Done;
    self.completed_at = Some(Utc::now());
    self.resolution = Some(Resolution::Completed);
    self.touch();

    Ok(())
  }

  pub fn complete_with_note(&mut self, note: impl Into<String>) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot complete a task that is already done"));
    }

    self.add_note(note);
    self.complete()
  }

  pub fn delegate(&mut self) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot delegate a task that is already done"));
    }

    self.stop();
    self.workflow_status = WorkflowStatus::Done;
    self.resolution = Some(Resolution::Delegated);
    self.touch();

    Ok(())
  }

  pub fn delegate_with_note(&mut self, note: impl Into<String>) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot delegate a task that is already done"));
    }

    self.add_note(note);
    self.delegate()
  }

  pub fn delete_note(&mut self, id: impl Into<String>) {
    let id = id.into();
    self.notes.retain(|n| n.id() != &id);
    self.touch();
  }

  pub fn has_tag(&self, label: impl Into<String>) -> bool {
    let labels: Vec<&String> = self.tags.iter().map(Tag::label).collect();
    labels.contains(&&label.into())
  }

  pub fn is_blocked(&self) -> bool {
    self.workflow_status == WorkflowStatus::Blocked
  }

  pub fn is_canceled(&self) -> bool {
    self.is_done() && self.resolution == Some(Resolution::Canceled)
  }

  pub fn is_complete(&self) -> bool {
    self.is_done() && self.resolution == Some(Resolution::Completed)
  }

  pub fn is_delegated(&self) -> bool {
    self.is_done() && self.resolution == Some(Resolution::Delegated)
  }

  pub fn is_done(&self) -> bool {
    self.workflow_status == WorkflowStatus::Done
  }

  pub fn is_external(&self) -> bool {
    self.external_id.is_some()
  }

  pub fn is_in_progress(&self) -> bool {
    self.workflow_status == WorkflowStatus::InProgress
  }

  pub fn is_todo(&self) -> bool {
    self.workflow_status == WorkflowStatus::Todo
  }

  pub fn log_work_from_source(
    &mut self,
    started_at: DateTime<Utc>,
    ended_at: DateTime<Utc>,
    source: impl Into<String>,
  ) {
    self.log_work_impl(started_at, ended_at, source, None);
  }

  pub fn log_work_from_source_with_note(
    &mut self,
    started_at: DateTime<Utc>,
    ended_at: DateTime<Utc>,
    source: impl Into<String>,
    note: impl Into<String>,
  ) {
    self.log_work_impl(started_at, ended_at, source, Some(note.into()));
  }

  pub fn move_to_project(&mut self, project: &Project) {
    self.project_id = Some(project.id().to_owned());
    self.touch();
  }

  pub fn start(&mut self) -> Result<()> {
    self.start_impl(None)
  }

  pub fn start_with_note(&mut self, note: impl Into<String>) -> Result<()> {
    self.start_impl(Some(note.into()))
  }

  pub fn stop(&mut self) {
    self.work_logs.iter_mut().for_each(WorkLog::stop);
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 15] {
    [
      &self.id,
      &self.project_id,
      &self.external_id,
      &self.title,
      &self.description,
      &self.priority,
      &self.workflow_status,
      &self.estimation,
      &self.resolution,
      &self.notes,
      &self.work_logs,
      &self.due_at,
      &self.completed_at,
      &self.created_at,
      &self.updated_at,
    ]
  }

  pub fn touch(&mut self) {
    self.updated_at = Utc::now();
  }

  pub fn unblock(&mut self) -> Result<()> {
    if self.is_blocked() {
      self.workflow_status = WorkflowStatus::Todo;
      self.touch();
      return Ok(());
    }

    Err(eyre!("Cannot unblock a task that is not blocked"))
  }

  pub fn update_description(&mut self, description: impl Into<String>) {
    self.description = Some(description.into());
    self.touch();
  }

  pub fn update_due_date(&mut self, due_at: DateTime<Utc>) -> Result<()> {
    if self.is_done() {
      return Err(eyre!(
        "Cannot update the due date of a task that is already done"
      ));
    }

    self.due_at = Some(due_at);
    self.touch();

    Ok(())
  }

  pub fn update_estimation(&mut self, estimation: Estimation) -> Result<()> {
    if self.is_done() {
      return Err(eyre!(
        "Cannot update the estimation of a task that is already done"
      ));
    }

    self.estimation = Some(estimation);
    self.touch();

    Ok(())
  }

  pub fn update_external_id(&mut self, external_id: impl Into<String>) {
    self.external_id = Some(external_id.into());
    self.touch();
  }

  pub fn update_note(&mut self, id: impl Into<String>, content: impl Into<String>) -> Result<()> {
    let id = id.into();
    let note = self
      .notes
      .iter_mut()
      .find(|n| n.id() == &id)
      .ok_or_else(|| eyre!("Note with id {} not found", id))?;

    note.update(content);
    self.touch();

    Ok(())
  }

  pub fn update_priority(&mut self, priority: Priority) -> Result<()> {
    if self.is_done() {
      return Err(eyre!(
        "Cannot update the priority of a task that is already done"
      ));
    }

    self.priority = priority;
    self.touch();

    Ok(())
  }

  #[must_use = "This method returns a new Task with the given description"]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  #[must_use = "This method returns a new Task with the given due date"]
  pub fn with_due_date(mut self, due_at: DateTime<Utc>) -> Self {
    self.due_at = Some(due_at);
    self
  }

  #[must_use = "This method returns a new Task with the given estimation"]
  pub fn with_estimation(mut self, estimation: Estimation) -> Self {
    self.estimation = Some(estimation);
    self
  }

  #[must_use = "This method returns a new Task with the given external id"]
  pub fn with_external_id(mut self, external_id: impl Into<String>) -> Self {
    self.external_id = Some(external_id.into());
    self
  }

  #[must_use = "This method returns a new Task with the given priority"]
  pub fn with_priority(mut self, priority: Priority) -> Self {
    self.priority = priority;
    self
  }

  #[must_use = "This method returns a new Task with the given project"]
  pub fn with_project(mut self, project: &Project) -> Self {
    self.project_id = Some(project.id().to_owned());
    self
  }

  #[must_use = "This method returns a new Task with the given workflow status"]
  pub fn with_workflow_status(mut self, workflow_status: WorkflowStatus) -> Self {
    self.workflow_status = workflow_status;
    self
  }

  fn log_work_impl(
    &mut self,
    started_at: DateTime<Utc>,
    ended_at: DateTime<Utc>,
    source: impl Into<String>,
    note: Option<String>,
  ) {
    let mut work_log = WorkLog::new()
      .with_start_date(started_at)
      .with_end_date(ended_at)
      .with_source(source);

    if let Some(note) = note {
      work_log = work_log.with_note(note);
    }

    self.work_logs.push(work_log);
    self.touch();
  }

  fn start_impl(&mut self, note: Option<String>) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot start a task that is already done"));
    }

    if self.is_blocked() {
      return Err(eyre!("Cannot start a task that is blocked"));
    }

    self.stop();
    let mut work_log = WorkLog::new();

    if let Some(note) = note {
      work_log = work_log.with_note(note);
    }

    self.work_logs.push(work_log);
    self.workflow_status = WorkflowStatus::InProgress;
    self.touch();

    Ok(())
  }
}

impl TryFrom<&Row<'_>> for Task {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      completed_at: row.get("completed_at")?,
      created_at: row.get("created_at")?,
      description: row.get("description")?,
      due_at: row.get("due_at")?,
      estimation: row.get("estimation")?,
      external_id: row.get("external_id")?,
      id: row.get("id")?,
      notes: row.get("notes")?,
      priority: row.get("priority")?,
      project_id: row.get("project_id")?,
      relationships: Vec::new(),
      resolution: row.get("resolution")?,
      sequence_id: row.get("sequence_id")?,
      tags: Vec::new(),
      title: row.get("title")?,
      updated_at: row.get("updated_at")?,
      work_logs: row.get("work_logs")?,
      workflow_status: row.get("workflow_status")?,
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod add_note {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_a_note() {
      let mut task = Task::new("a test task");
      task.add_note("a note");

      assert_eq!(task.notes.len(), 1);
      assert_eq!(task.notes[0].content(), "a note");
    }
  }

  mod block {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_blocks_a_task() {
      let mut task = Task::new("a test task");
      task.block().unwrap();

      assert_eq!(task.workflow_status, WorkflowStatus::Blocked);
    }

    #[test]
    fn it_returns_an_error_if_the_task_is_already_done() {
      let mut task = Task::new("a test task");
      task.complete().unwrap();

      assert!(task.block().is_err());
    }
  }

  mod block_with_note {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_blocks_a_task_with_a_note() {
      let mut task = Task::new("a test task");
      task.block_with_note("a note").unwrap();

      assert_eq!(task.workflow_status, WorkflowStatus::Blocked);
      assert_eq!(task.notes.len(), 1);
      assert_eq!(task.notes[0].content(), "a note");
    }
  }

  mod cancel {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_cancels_a_task() {
      let mut task = Task::new("a test task");
      task.cancel().unwrap();

      assert_eq!(task.workflow_status, WorkflowStatus::Done);
      assert_eq!(task.resolution, Some(Resolution::Canceled));
    }

    #[test]
    fn it_returns_an_error_if_the_task_is_already_done() {
      let mut task = Task::new("a test task");
      task.complete().unwrap();

      assert!(task.cancel().is_err());
    }
  }

  mod cancel_with_note {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_cancels_a_task_with_a_note() {
      let mut task = Task::new("a test task");
      task.cancel_with_note("a note").unwrap();

      assert_eq!(task.workflow_status, WorkflowStatus::Done);
      assert_eq!(task.resolution, Some(Resolution::Canceled));
      assert_eq!(task.notes.len(), 1);
    }
  }

  mod complete {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_completes_a_task() {
      let mut task = Task::new("a test task");
      task.complete().unwrap();

      assert!(task.completed_at().unwrap().timestamp() > 0);
      assert_eq!(task.workflow_status, WorkflowStatus::Done);
      assert_eq!(task.resolution, Some(Resolution::Completed));
    }

    #[test]
    fn it_returns_an_error_if_the_task_is_already_done() {
      let mut task = Task::new("a test task");
      task.complete().unwrap();

      assert!(task.complete().is_err());
    }
  }

  mod complete_with_note {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_completes_a_task_with_a_note() {
      let mut task = Task::new("a test task");
      task.complete_with_note("a note").unwrap();

      assert_eq!(task.notes().len(), 1);
      assert_eq!(task.notes()[0].content(), "a note");
      assert!(task.completed_at().unwrap().timestamp() > 0);
      assert_eq!(task.workflow_status, WorkflowStatus::Done);
      assert_eq!(task.resolution, Some(Resolution::Completed));
    }
  }

  mod delegate {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_delegates_a_task() {
      let mut task = Task::new("a test task");
      task.delegate().unwrap();

      assert_eq!(task.workflow_status, WorkflowStatus::Done);
      assert_eq!(task.resolution, Some(Resolution::Delegated));
    }

    #[test]
    fn it_returns_an_error_if_the_task_is_already_done() {
      let mut task = Task::new("a test task");
      task.complete().unwrap();

      assert!(task.delegate().is_err());
    }
  }

  mod delegate_with_note {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_delegates_a_task_with_a_note() {
      let mut task = Task::new("a test task");
      task.delegate_with_note("a note").unwrap();

      assert_eq!(task.notes().len(), 1);
      assert_eq!(task.notes()[0].content(), "a note");
      assert_eq!(task.workflow_status, WorkflowStatus::Done);
      assert_eq!(task.resolution, Some(Resolution::Delegated));
    }
  }

  mod delete_note {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_deletes_a_note() {
      let mut task = Task::new("a test task");
      task.add_note("a note");
      let id = &task.notes[0].id();
      task.delete_note((*id).clone());

      assert_eq!(task.notes.len(), 0);
    }
  }

  mod has_tag {
    use super::*;

    #[test]
    fn it_returns_true_if_the_task_has_the_tag() {
      let mut task = Task::new("a test task");
      task.tags.push(Tag::new("test"));

      assert!(task.has_tag("test"));
    }

    #[test]
    fn it_returns_false_if_the_task_does_not_have_the_tag() {
      let task = Task::new("a test tag");

      assert!(!task.has_tag("test"));
    }
  }

  mod is_blocked {
    use super::*;

    #[test]
    fn it_returns_true_if_the_task_is_blocked() {
      let mut task = Task::new("a test task");
      task.workflow_status = WorkflowStatus::Blocked;

      assert!(task.is_blocked());
    }

    #[test]
    fn it_returns_false_if_the_task_is_not_blocked() {
      let task = Task::new("a test task");

      assert!(!task.is_blocked());
    }
  }

  mod is_canceled {
    use super::*;

    #[test]
    fn it_returns_true_if_the_task_is_canceled() {
      let mut task = Task::new("a test task");
      task.workflow_status = WorkflowStatus::Done;
      task.resolution = Some(Resolution::Canceled);

      assert!(task.is_canceled());
    }

    #[test]
    fn it_returns_false_if_the_task_is_not_canceled() {
      let task = Task::new("a test task");

      assert!(!task.is_canceled());
    }
  }

  mod is_complete {
    use super::*;

    #[test]
    fn it_returns_true_if_the_task_is_complete() {
      let mut task = Task::new("a test task");
      task.workflow_status = WorkflowStatus::Done;
      task.resolution = Some(Resolution::Completed);

      assert!(task.is_complete());
    }

    #[test]
    fn it_returns_false_if_the_task_is_not_complete() {
      let task = Task::new("a test task");

      assert!(!task.is_complete());
    }
  }

  mod is_delegated {
    use super::*;

    #[test]
    fn it_returns_true_if_the_task_is_delegated() {
      let mut task = Task::new("a test task");
      task.workflow_status = WorkflowStatus::Done;
      task.resolution = Some(Resolution::Delegated);

      assert!(task.is_delegated());
    }

    #[test]
    fn it_returns_false_if_the_task_is_not_delegated() {
      let task = Task::new("a test task");

      assert!(!task.is_delegated());
    }
  }

  mod is_done {
    use super::*;

    #[test]
    fn it_returns_true_if_the_task_is_done() {
      let mut task = Task::new("a test task");
      task.workflow_status = WorkflowStatus::Done;

      assert!(task.is_done());
    }

    #[test]
    fn it_returns_false_if_the_task_is_not_done() {
      let task = Task::new("a test task");

      assert!(!task.is_done());
    }
  }

  mod is_external {
    use super::*;

    #[test]
    fn it_returns_true_if_the_task_is_external() {
      let mut task = Task::new("a test task");
      task.external_id = Some("123".to_owned());

      assert!(task.is_external());
    }

    #[test]
    fn it_returns_false_if_the_task_is_not_external() {
      let task = Task::new("a test task");

      assert!(!task.is_external());
    }
  }

  mod is_in_progress {
    use super::*;

    #[test]
    fn it_returns_true_if_the_task_is_in_progress() {
      let mut task = Task::new("a test task");
      task.workflow_status = WorkflowStatus::InProgress;

      assert!(task.is_in_progress());
    }

    #[test]
    fn it_returns_false_if_the_task_is_not_in_progress() {
      let task = Task::new("a test task");

      assert!(!task.is_in_progress());
    }
  }

  mod is_todo {
    use super::*;

    #[test]
    fn it_returns_true_if_the_task_is_todo() {
      let task = Task::new("a test task");

      assert!(task.is_todo());
    }

    #[test]
    fn it_returns_false_if_the_task_is_not_todo() {
      let mut task = Task::new("a test task");
      task.workflow_status = WorkflowStatus::Done;

      assert!(!task.is_todo());
    }
  }

  mod log_work_from_source {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_logs_work_from_source() {
      let mut task = Task::new("a test task");
      let now = Utc::now();
      task.log_work_from_source(now, now, "git");

      assert_eq!(task.work_logs().len(), 1);

      let work_log = &task.work_logs()[0];

      assert_eq!(work_log.started_at(), &now);
      assert_eq!(work_log.ended_at(), Some(now));
      assert_eq!(work_log.source(), Some("git".to_string()));
    }
  }

  mod log_work_from_source_with_note {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_logs_work_from_source_with_note() {
      let mut task = Task::new("a test task");
      let now = Utc::now();
      task.log_work_from_source_with_note(now, now, "git", "a note");

      assert_eq!(task.work_logs().len(), 1);

      let work_log = &task.work_logs()[0];

      assert_eq!(work_log.started_at(), &now);
      assert_eq!(work_log.ended_at(), Some(now));
      assert_eq!(work_log.note(), Some("a note".to_string()));
      assert_eq!(work_log.source(), Some("git".to_string()));
    }
  }

  mod move_to_project {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_assigns_the_task_to_the_project() {
      let project = Project::new("A test project");
      let mut task = Task::new("a test task");
      task.move_to_project(&project);

      assert_eq!(task.project_id(), Some(project.id().clone()));
    }
  }

  mod start {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_creates_a_starts_the_task() {
      let mut task = Task::new("a test task");
      task.start().unwrap();

      assert_eq!(task.work_logs().len(), 1);
      assert_eq!(task.workflow_status, WorkflowStatus::InProgress);
    }

    #[test]
    fn it_stops_previous_work_logs() {
      let mut task = Task::new("a test task");
      task.start().unwrap();
      task.start().unwrap();

      assert_eq!(task.work_logs().len(), 2);
      assert_eq!(
        task
          .work_logs()
          .iter()
          .filter(|wl| wl.ended_at().is_none())
          .count(),
        1
      );
    }

    #[test]
    fn it_returns_an_error_if_the_task_is_already_done() {
      let mut task = Task::new("a test task");
      task.complete().unwrap();

      assert!(task.start().is_err());
    }

    #[test]
    fn it_returns_an_error_if_the_task_is_blocked() {
      let mut task = Task::new("a test task");
      task.block().unwrap();

      assert!(task.start().is_err());
    }
  }

  mod start_with_note {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_starts_the_task_with_a_note() {
      let mut task = Task::new("a test task");
      task.start_with_note("a note").unwrap();

      assert_eq!(task.work_logs().len(), 1);
      assert_eq!(task.workflow_status, WorkflowStatus::InProgress);
      assert_eq!(task.work_logs()[0].note(), Some("a note".to_string()));
    }
  }

  mod stop {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_stops_the_task() {
      let mut task = Task::new("a test task");
      task.start().unwrap();

      assert!(task.work_logs().iter().all(|wl| wl.ended_at().is_none()));

      task.stop();

      assert_eq!(task.work_logs().len(), 1);
      assert!(task.work_logs().iter().all(|wl| wl.ended_at().is_some()));
    }
  }

  mod to_sql_params {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_returns_the_project_fields_as_sql_params() {
      let connection = Connection::open_in_memory().unwrap();
      connection
        .execute(
          r"
        CREATE TABLE tasks (
          id TEXT PRIMARY KEY,
          sequence_id INTEGER,
          project_id TEXT,
          external_id TEXT,
          title TEXT NOT NULL,
          description TEXT,
          priority TEXT NOT NULL,
          workflow_status TEXT NOT NULL,
          estimation TEXT,
          resolution TEXT,
          notes TEXT NOT NULL DEFAULT '[]',
          work_logs TEXT NOT NULL DEFAULT '[]',
          due_at TIMESTAMP,
          completed_at TIMESTAMP,
          created_at TIMESTAMP NOT NULL,
          updated_at TIMESTAMP NOT NULL
        )
      ",
          [],
        )
        .unwrap();

      let task = Task::new("a test task");

      let result = connection.execute(
        r"
        INSERT INTO tasks (
          id,
          project_id,
          external_id,
          title,
          description,
          priority,
          workflow_status,
          estimation,
          resolution,
          notes,
          work_logs,
          due_at,
          completed_at,
          created_at,
          updated_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
      ",
        task.to_sql_params(),
      );

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), 1);
    }
  }

  mod touch {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_updates_the_tasks_updated_at_field() {
      let mut task = Task::new("a test task");
      let old_updated_at = Utc::now() - chrono::Duration::days(2);
      task.updated_at = old_updated_at;

      assert_eq!(task.updated_at(), &old_updated_at);

      task.touch();

      assert_ne!(task.updated_at(), &old_updated_at);
      assert!(task.updated_at() > &old_updated_at);
    }
  }

  mod try_from_row {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_converts_a_row_into_a_task() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection
        .prepare(
          r"
        SELECT
          null AS completed_at,
          datetime('now', 'utc') AS created_at,
          'a description' AS description,
          null AS due_at,
          '2 points' AS estimation,
          null AS external_id,
          'aq9igqkz4lv2c6nodxd6ndi2' as id,
          '[]' AS notes,
          'p2' AS priority,
          null AS project_id,
          null AS resolution,
          1 AS sequence_id,
          'a test task' AS title,
          datetime('now', 'utc') AS updated_at,
          '[]' AS work_logs,
          'todo' AS workflow_status
      ",
        )
        .unwrap();
      let task = statement.query_row([], |row| Task::try_from(row)).unwrap();

      assert_eq!(task.id(), "aq9igqkz4lv2c6nodxd6ndi2");
    }
  }

  mod unblock {

    use super::*;

    #[test]
    fn it_unblocks_the_task() {
      let mut task = Task::new("a test task");
      task.block().unwrap();

      assert!(task.is_blocked());

      task.unblock().unwrap();

      assert!(!task.is_blocked());
    }

    #[test]
    fn it_cant_unblock_a_task_that_is_not_blocked() {
      let mut task = Task::new("a test task");

      assert!(task.unblock().is_err());
    }
  }

  mod update_description {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_tasks_description() {
      let mut task = Task::new("a test task").with_description("old");
      task.update_description("new");

      assert_eq!(task.description(), Some("new".to_string()));
    }
  }

  mod update_due_date {
    use super::*;

    #[test]
    fn it_updates_the_tasks_due_at() {
      let mut task = Task::new("a test task");
      task.update_due_date(Utc::now()).unwrap();

      assert!(task.due_at().unwrap().timestamp() > 0);
    }

    #[test]
    fn it_cannot_update_the_task_if_it_is_already_done() {
      let mut task = Task::new("a test task");
      task.complete().unwrap();

      assert!(task.update_due_date(Utc::now()).is_err());
    }
  }

  mod update_estimation {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_task_estimation() {
      let mut task = Task::new("a test task");
      task.update_estimation(Estimation::Points(2)).unwrap();

      assert_eq!(task.estimation(), Some(Estimation::Points(2)));
    }

    #[test]
    fn it_cannot_update_the_task_if_it_is_already_done() {
      let mut task = Task::new("a test task");
      task.complete().unwrap();

      assert!(task.update_estimation(Estimation::Points(2)).is_err());
    }
  }

  mod update_external_id {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_tasks_external_id() {
      let mut task = Task::new("a test task");
      task.update_external_id("123");

      assert_eq!(task.external_id(), Some("123".to_string()));
    }
  }

  mod update_note {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_a_task_note() {
      let mut task = Task::new("a test task");
      task.add_note("old content");
      let id = &task.notes[0].id().clone();
      task.update_note(id, "new content").unwrap();

      assert_eq!(task.notes[0].content(), "new content");
    }
  }

  mod update_priority {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_tasks_priority() {
      let mut task = Task::new("a test task");
      task.update_priority(Priority::Critical).unwrap();

      assert_eq!(task.priority(), &Priority::Critical);
    }

    #[test]
    fn it_cannot_update_the_task_if_it_is_already_done() {
      let mut task = Task::new("a test task");
      task.complete().unwrap();

      assert!(task.update_priority(Priority::Critical).is_err());
    }
  }

  mod with_description {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_task_with_the_description() {
      let task = Task::new("a test task").with_description("a description");

      assert_eq!(task.description(), Some("a description".to_string()));
    }
  }

  mod with_due_date {
    use super::*;

    #[test]
    fn it_returns_a_new_task_with_the_due_date() {
      let task = Task::new("a test task").with_due_date(Utc::now());

      assert!(task.due_at().unwrap().timestamp() > 0);
    }
  }

  mod with_estimation {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_task_with_the_estimation() {
      let task = Task::new("a test task").with_estimation(Estimation::Points(2));

      assert_eq!(task.estimation(), Some(Estimation::Points(2)));
    }
  }

  mod with_external_id {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_task_with_the_external_id() {
      let task = Task::new("a test task").with_external_id("123");

      assert_eq!(task.external_id(), Some("123".to_string()));
    }
  }

  mod with_priority {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_task_with_the_priority() {
      let task = Task::new("a test task").with_priority(Priority::Critical);

      assert_eq!(task.priority(), &Priority::Critical);
    }
  }

  mod with_project {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_task_with_the_project() {
      let project = Project::new("A test project");
      let task = Task::new("a test task").with_project(&project);

      assert_eq!(task.project_id(), Some(project.id().clone()));
    }
  }

  mod with_workflow_status {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_task_with_the_workflow_status() {
      let task = Task::new("a test task").with_workflow_status(WorkflowStatus::InProgress);

      assert_eq!(task.workflow_status, WorkflowStatus::InProgress);
    }
  }
}
