use chrono::Utc;
use eyre::Result;

use super::constructor::*;
use crate::Project;

impl<'a> Repo<'a> {
  pub fn create(&self, project: Project) -> Result<Project> {
    self.save(project)
  }

  pub fn update(&self, mut project: Project) -> Result<Project> {
    project.set_updated_at(Utc::now());
    self.save(project)
  }

  pub(crate) fn save(&self, project: Project) -> Result<Project> {
    let mut statement = self.connection.prepare(UPSERT_SQL)?;
    statement.execute(project.to_sql_params())?;

    self.by_pk(project.id())
  }
}

const UPSERT_SQL: &str = r"
  INSERT INTO projects (
    id,
    name,
    key,
    description,
    workflow_status,
    resolution,
    completed_at,
    created_at,
    updated_at
  )
  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
  ON CONFLICT (id) DO UPDATE SET
    name = excluded.name,
    key = excluded.key,
    description = excluded.description,
    workflow_status = excluded.workflow_status,
    resolution = excluded.resolution,
    completed_at = excluded.completed_at,
    updated_at = excluded.updated_at
";
