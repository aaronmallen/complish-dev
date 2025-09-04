use eyre::Result;

use super::constructor::*;
use crate::{Project, ProjectUpdate, Tag, Task};

impl<'a> Repo<'a> {
  pub(crate) fn hydrate(&self, mut project: Project) -> Result<Project> {
    project.tags = self.tags(project.id())?;
    project.tasks = self.tasks(project.id())?;
    project.updates = self.updates(project.id())?;

    Ok(project)
  }

  fn tags(&self, project_id: impl Into<String>) -> Result<Vec<Tag>> {
    let mut statement = self.connection.prepare(TAG_BY_PROJECT_ID_SQL)?;
    statement
      .query_map([project_id.into()], |row| Tag::try_from(row))?
      .map(|r| r.map_err(Into::into))
      .collect()
  }

  fn tasks(&self, project_id: impl Into<String>) -> Result<Vec<Task>> {
    let mut statement = self.connection.prepare(TASK_BY_PROJECT_ID_SQL)?;
    statement
      .query_map([project_id.into()], |row| Task::try_from(row))?
      .map(|r| r.map_err(Into::into))
      .collect()
  }

  fn updates(&self, project_id: impl Into<String>) -> Result<Vec<ProjectUpdate>> {
    let mut statement = self.connection.prepare(UPDATE_BY_PROJECT_ID_SQL)?;
    statement
      .query_map([project_id.into()], |row| ProjectUpdate::try_from(row))?
      .map(|r| r.map_err(Into::into))
      .collect()
  }
}

const TAG_BY_PROJECT_ID_SQL: &str = r"
  SELECT
    tags.*
  FROM tags
  JOIN project_tags
    ON tags.id = project_tags.tag_id
  WHERE project_tags.project_id = ?1
";

const TASK_BY_PROJECT_ID_SQL: &str = "SELECT * FROM tasks WHERE project_id = ?1";

const UPDATE_BY_PROJECT_ID_SQL: &str = r"
  SELECT *
  FROM project_updates
  WHERE project_id = ?1
  ORDER BY created_at
";
