use chrono::Utc;
use eyre::Result;

use super::constructor::*;
use crate::{Project, ProjectUpdate, ProjectUpdateStatus};

impl<'a> Repo<'a> {
  pub fn add_update(&self, project: Project, status: ProjectUpdateStatus) -> Result<Project> {
    self.add_update_impl(project, status, None)
  }

  pub fn add_update_with_description(
    &self,
    project: Project,
    status: ProjectUpdateStatus,
    description: impl Into<String>,
  ) -> Result<Project> {
    self.add_update_impl(project, status, Some(description.into()))
  }

  pub fn delete_update(&self, update_id: impl Into<String>) -> Result<Project> {
    let update = self.find_update(update_id)?;
    let mut statement = self.connection.prepare(DELETE_SQL)?;
    statement.execute([update.id()])?;

    self.by_pk(update.project_id())
  }

  pub fn update_update_description(
    &self,
    update_id: impl Into<String>,
    description: impl Into<String>,
  ) -> Result<Project> {
    let mut update = self.find_update(update_id)?;

    update.set_description(Some(description.into()));
    update.set_updated_at(Utc::now());
    self.save_update(update)
  }

  pub fn update_update_status(
    &self,
    update_id: impl Into<String>,
    status: ProjectUpdateStatus,
  ) -> Result<Project> {
    let mut update = self.find_update(update_id)?;

    update.set_status(status);
    update.set_updated_at(Utc::now());
    self.save_update(update)
  }

  fn add_update_impl(
    &self,
    project: Project,
    status: ProjectUpdateStatus,
    description: Option<String>,
  ) -> Result<Project> {
    let mut update = ProjectUpdate::new(project.id(), status);
    if let Some(description) = description {
      update = update.with_description(description)
    };

    let mut statement = self.connection.prepare(UPSERT_SQL)?;
    statement.execute(update.to_sql_params())?;

    self.by_pk(project.id())
  }

  fn find_update(&self, update_id: impl Into<String>) -> Result<ProjectUpdate> {
    let mut statement = self.connection.prepare(UPDATE_BY_PK_SQL)?;
    statement
      .query_row([update_id.into()], |row| ProjectUpdate::try_from(row))
      .map_err(Into::into)
  }

  fn save_update(&self, update: ProjectUpdate) -> Result<Project> {
    let mut statement = self.connection.prepare(UPSERT_SQL)?;
    statement.execute(update.to_sql_params())?;

    self.by_pk(update.project_id())
  }
}

const DELETE_SQL: &str = "DELETE FROM project_updates WHERE id = ?1";

const UPDATE_BY_PK_SQL: &str = "SELECT * FROM project_updates WHERE id = ?1";

const UPSERT_SQL: &str = r"
  INSERT INTO project_updates (id, project_id, description, status, created_at, updated_at)
  VALUES (?1, ?2, ?3, ?4, ?5, ?6)
  ON CONFLICT (id) DO UPDATE SET
    description = excluded.description,
    status = excluded.status,
    updated_at = excluded.updated_at
";
