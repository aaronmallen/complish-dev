pub(crate) mod resolution;
pub(crate) mod update;
pub(crate) mod update_status;
pub(crate) mod workflow_status;

use std::str::FromStr;

use chrono::{DateTime, Utc};
use cuid::cuid2;
use eyre::{Result, eyre};
use getset::{CloneGetters, Getters};
use resolution::Resolution;
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};
use update::Update;
use workflow_status::WorkflowStatus;

use crate::{
  entities::{RGB, Tag, Task},
  types::JsonVec,
};

#[derive(Clone, CloneGetters, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct Project {
  #[get = "pub"]
  color: RGB,
  #[get_clone = "pub"]
  completed_at: Option<DateTime<Utc>>,
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get_clone = "pub"]
  description: Option<String>,
  #[get = "pub"]
  id: String,
  #[get = "pub"]
  key: String,
  #[get = "pub"]
  name: String,
  #[get_clone = "pub"]
  resolution: Option<Resolution>,
  #[get = "pub"]
  pub(crate) tags: Vec<Tag>,
  #[get = "pub"]
  pub(crate) tasks: Vec<Task>,
  #[get = "pub"]
  updates: JsonVec<Update>,
  #[get = "pub"]
  updated_at: DateTime<Utc>,
  #[get = "pub"]
  workflow_status: WorkflowStatus,
}

impl Project {
  pub fn new(name: impl Into<String>) -> Self {
    let name = name.into();
    let now = Utc::now();

    let key = if name.len() >= 3 {
      name[0..3].to_uppercase()
    } else {
      name.to_uppercase()
    };

    Self {
      color: RGB::random(),
      completed_at: None,
      created_at: now,
      description: None,
      id: cuid2(),
      key,
      name,
      resolution: None,
      tags: Vec::new(),
      tasks: Vec::new(),
      updates: JsonVec::new(),
      updated_at: now,
      workflow_status: WorkflowStatus::default(),
    }
  }

  pub fn add_update(&mut self, update: Update) {
    self.updates.push(update);
    self.touch();
  }

  pub fn cancel(&mut self) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot cancel a project that is already done"));
    }

    self.resolution = Some(Resolution::Canceled);
    self.workflow_status = WorkflowStatus::Done;
    self.touch();
    Ok(())
  }

  pub fn complete(&mut self) -> Result<()> {
    if self.is_done() {
      return Err(eyre!("Cannot complete a project that is already done"));
    }

    self.completed_at = Some(Utc::now());
    self.resolution = Some(Resolution::Completed);
    self.workflow_status = WorkflowStatus::Done;
    self.touch();
    Ok(())
  }

  pub fn has_tag(&self, label: impl Into<String>) -> bool {
    let labels: Vec<&String> = self.tags.iter().map(Tag::label).collect();
    labels.contains(&&label.into())
  }

  pub fn is_canceled(&self) -> bool {
    self.is_done() && self.resolution == Some(Resolution::Canceled)
  }

  pub fn is_complete(&self) -> bool {
    self.is_done() && self.resolution == Some(Resolution::Completed)
  }

  pub fn is_done(&self) -> bool {
    self.workflow_status == WorkflowStatus::Done
  }

  pub fn is_planned(&self) -> bool {
    self.workflow_status == WorkflowStatus::Planned
  }

  pub fn is_in_progress(&self) -> bool {
    self.workflow_status == WorkflowStatus::InProgress
  }

  pub fn is_todo(&self) -> bool {
    self.workflow_status == WorkflowStatus::Todo
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 11] {
    [
      &self.id,
      &self.name,
      &self.key,
      &self.description,
      &self.workflow_status,
      &self.resolution,
      &self.updates,
      &self.color,
      &self.completed_at,
      &self.created_at,
      &self.updated_at,
    ]
  }

  pub fn touch(&mut self) {
    self.updated_at = Utc::now();
  }

  pub fn update_color(&mut self, red: u8, green: u8, blue: u8) {
    self.color = RGB::new(red, green, blue);
    self.touch();
  }

  pub fn update_description(&mut self, description: impl Into<String>) {
    self.description = Some(description.into());
    self.touch();
  }

  pub fn update_hex_color(&mut self, hex: impl Into<String>) -> Result<()> {
    self.color = RGB::from_str(hex.into().as_str())?;
    self.touch();
    Ok(())
  }

  pub fn update_key(&mut self, key: impl Into<String>) {
    self.key = key.into();
    self.touch();
  }

  #[must_use = "This method returns a new Project with the color set"]
  pub fn with_color(mut self, red: u8, green: u8, blue: u8) -> Self {
    self.color = RGB::new(red, green, blue);
    self
  }

  #[must_use = "This method returns a new Project with the description set"]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  pub fn with_hex_color(mut self, hex: impl Into<String>) -> Result<Self> {
    self.color = RGB::from_str(hex.into().as_str())?;
    Ok(self)
  }

  #[must_use = "This method returns a new Project with the key set"]
  pub fn with_key(mut self, key: impl Into<String>) -> Self {
    self.key = key.into();
    self
  }

  #[must_use = "This method returns a new Project with the work flow status set"]
  pub fn with_workflow_status(mut self, status: WorkflowStatus) -> Self {
    self.workflow_status = status;
    self
  }
}

impl TryFrom<&Row<'_>> for Project {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      color: row.get("color")?,
      completed_at: row.get("completed_at")?,
      created_at: row.get("created_at")?,
      description: row.get("description")?,
      id: row.get("id")?,
      key: row.get("key")?,
      name: row.get("name")?,
      resolution: row.get("resolution")?,
      tags: Vec::new(),
      tasks: Vec::new(),
      updates: row.get("updates")?,
      updated_at: row.get("updated_at")?,
      workflow_status: row.get("workflow_status")?,
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod add_update {
    use pretty_assertions::assert_eq;

    use super::{update_status::UpdateStatus, *};

    #[test]
    fn it_adds_updates() {
      let mut project = Project::new("Test Project");
      let update = Update::new(UpdateStatus::OnTrack);
      project.add_update(update.clone());

      assert_eq!(project.updates().len(), 1);
      assert_eq!(project.updates()[0], update);
    }
  }

  mod cancel {
    use super::*;

    #[test]
    fn it_cancels_the_project() {
      let mut project = Project::new("Test Project");
      project.cancel().unwrap();

      assert!(project.is_canceled());
    }

    #[test]
    fn it_cannot_cancel_a_done_project() {
      let mut project = Project::new("Test Project");
      project.complete().unwrap();

      assert!(project.cancel().is_err());
    }
  }

  mod complete {
    use super::*;

    #[test]
    fn it_completes_the_project() {
      let mut project = Project::new("Test Project");
      project.complete().unwrap();

      assert!(project.is_complete());
      assert!(project.completed_at().unwrap().timestamp() > 0);
    }

    #[test]
    fn it_cannot_complete_a_done_project() {
      let mut project = Project::new("Test Project");
      project.complete().unwrap();

      assert!(project.complete().is_err());
    }
  }

  mod has_tag {
    use super::*;

    #[test]
    fn it_returns_true_if_the_project_has_the_tag() {
      let mut project = Project::new("Test Project");
      project.tags.push(Tag::new("test"));

      assert!(project.has_tag("test"));
    }

    #[test]
    fn it_returns_false_if_the_project_does_not_have_the_tag() {
      let project = Project::new("Test Project");

      assert!(!project.has_tag("test"));
    }
  }

  mod is_canceled {
    use super::*;

    #[test]
    fn it_returns_true_if_the_project_is_canceled() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::Done;
      project.resolution = Some(Resolution::Canceled);

      assert!(project.is_canceled());
    }

    #[test]
    fn it_returns_false_if_the_project_is_not_canceled() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::Done;
      project.resolution = Some(Resolution::Completed);

      assert!(!project.is_canceled());
    }
  }

  mod is_complete {
    use super::*;

    #[test]
    fn it_returns_true_if_the_project_is_complete() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::Done;
      project.resolution = Some(Resolution::Completed);

      assert!(project.is_complete());
    }

    #[test]
    fn it_returns_false_if_the_project_is_not_complete() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::Done;
      project.resolution = Some(Resolution::Canceled);

      assert!(!project.is_complete());
    }
  }

  mod is_done {
    use super::*;

    #[test]
    fn it_returns_true_if_the_project_is_done() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::Done;

      assert!(project.is_done());
    }

    #[test]
    fn it_returns_false_if_the_project_is_not_done() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::Todo;

      assert!(!project.is_done());
    }
  }

  mod is_planned {
    use super::*;

    #[test]
    fn it_returns_true_if_the_project_is_planned() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::Planned;

      assert!(project.is_planned());
    }

    #[test]
    fn it_returns_false_if_the_project_is_not_planned() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::Todo;

      assert!(!project.is_planned());
    }
  }

  mod is_in_progress {
    use super::*;

    #[test]
    fn it_returns_true_if_the_project_is_in_progress() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::InProgress;

      assert!(project.is_in_progress());
    }

    #[test]
    fn it_returns_false_if_the_project_is_not_in_progress() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::Todo;

      assert!(!project.is_in_progress());
    }
  }

  mod is_todo {
    use super::*;

    #[test]
    fn it_returns_true_if_the_project_is_todo() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::Todo;

      assert!(project.is_todo());
    }

    #[test]
    fn it_returns_false_if_the_project_is_not_todo() {
      let mut project = Project::new("Test Project");
      project.workflow_status = WorkflowStatus::InProgress;

      assert!(!project.is_todo());
    }
  }

  mod new {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_creates_a_new_project() {
      let project = Project::new("Test Project");

      assert_eq!(project.name(), "Test Project");
      assert_eq!(project.key(), "TES");
      assert!(project.created_at().timestamp() > 0);
      assert!(project.updated_at().timestamp() > 0);
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
        CREATE TABLE projects (
          id TEXT PRIMARY KEY,
          name TEXT NOT NULL,
          key TEXT NOT NULL,
          description TEXT,
          workflow_status TEXT NOT NULL,
          resolution TEXT,
          updates TEXT NOT NULL,
          color TEXT NOT NULL,
          completed_at TIMESTAMP,
          created_at TIMESTAMP NOT NULL,
          updated_at TIMESTAMP NOT NULL
        )
      ",
          [],
        )
        .unwrap();

      let project = Project::new("Test Project");

      let result = connection.execute(r"
        INSERT INTO projects (id, name, key, description, workflow_status, resolution, updates, color, completed_at, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
      ", project.to_sql_params());

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), 1);
    }
  }

  mod touch {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_updates_the_updated_at_field() {
      let mut project = Project::new("Test Project");
      let old_updated_at = Utc::now() - chrono::Duration::days(1);
      project.updated_at = old_updated_at;

      assert_eq!(&old_updated_at, project.updated_at());

      project.touch();

      assert_ne!(&old_updated_at, project.updated_at());
      assert!(project.updated_at() > &old_updated_at);
    }
  }

  mod try_from_row {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_converts_a_row_into_a_project() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection
        .prepare(
          r"
        SELECT
          'popmfy0xiz8ykp3urgojxtma' as id,
          'Test Project' as name,
          'TES' as key,
          'test description' as description,
          'todo' as workflow_status,
          null as resolution,
          '[]' as updates,
          '#FF0000' as color,
          null as completed_at,
          datetime('now', 'utc') as created_at,
          datetime('now', 'utc') as updated_at
      ",
        )
        .unwrap();
      let project = statement
        .query_row([], |row| Project::try_from(row))
        .unwrap();

      assert_eq!(project.id(), "popmfy0xiz8ykp3urgojxtma");
      assert_eq!(project.name(), "Test Project");
      assert_eq!(project.key(), "TES");
      assert_eq!(project.description(), Some("test description".to_string()));
      assert_eq!(project.workflow_status(), &WorkflowStatus::Todo);
      assert_eq!(project.resolution(), None);
      assert_eq!(project.updates(), &JsonVec::new());
      assert_eq!(project.color(), &RGB::new(255, 0, 0));
      assert_eq!(project.completed_at(), None);
      assert!(project.created_at().timestamp() > 0);
      assert!(project.updated_at().timestamp() > 0);
    }
  }

  mod update_color {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_color() {
      let mut project = Project::new("Test Project").with_color(0, 255, 0);
      let old_color = project.color().clone();
      project.update_color(255, 0, 0);

      assert_eq!(project.color(), &RGB::new(255, 0, 0));
      assert_ne!(project.color(), &old_color);
    }
  }

  mod update_description {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_updates_the_description() {
      let mut project = Project::new("Test Project");
      let old_description = project.description();
      project.update_description("Updated description");

      assert_ne!(project.description(), old_description);
      assert_eq!(
        project.description(),
        Some("Updated description".to_string())
      );
    }
  }

  mod update_hex_color {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_hex_color() {
      let mut project = Project::new("Test Project");
      let old_color = project.color().clone();
      project.update_hex_color("#FF0000").unwrap();

      assert_eq!(project.color(), &RGB::new(255, 0, 0));
      assert_ne!(project.color(), &old_color);
    }

    #[test]
    fn it_returns_an_error_if_the_hex_color_is_invalid() {
      let mut project = Project::new("Test Project");

      assert!(project.update_hex_color("#GGGGGGG").is_err());
    }
  }

  mod update_key {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_key() {
      let mut project = Project::new("Test Project");
      let old_key = project.key().clone();
      project.update_key("Updated key");

      assert_ne!(project.key(), &old_key);
      assert_eq!(project.key(), "Updated key");
    }
  }

  mod with_color {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_project_with_the_color_set() {
      let project = Project::new("Test Project").with_color(0, 255, 0);

      assert_eq!(project.color(), &RGB::new(0, 255, 0));
    }
  }

  mod with_description {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_project_with_the_description_set() {
      let project = Project::new("Test Project").with_description("Test description");

      assert_eq!(project.description(), Some("Test description".to_string()));
    }
  }

  mod with_hex_color {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_project_with_the_hex_color() {
      let project = Project::new("Test Project")
        .with_hex_color("#FF0000")
        .unwrap();

      assert_eq!(project.color(), &RGB::new(255, 0, 0));
    }

    #[test]
    fn it_returns_an_error_if_the_hex_color_is_invalid() {
      let project = Project::new("Test Project").with_hex_color("#GGGGGGG");

      assert!(project.is_err());
    }
  }

  mod with_key {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_project_with_the_key_set() {
      let project = Project::new("Test Project").with_key("Updated key");

      assert_eq!(project.key(), "Updated key");
    }
  }

  mod with_workflow_status {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_project_with_the_workflow_status_set() {
      let project = Project::new("Test Project").with_workflow_status(WorkflowStatus::Planned);

      assert_eq!(project.workflow_status(), &WorkflowStatus::Planned);
    }
  }
}
