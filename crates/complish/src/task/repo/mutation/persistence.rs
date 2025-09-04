use chrono::Utc;
use eyre::Result;

use super::constructor::*;
use crate::Task;

impl<'a> Repo<'a> {
  pub fn create(&self, task: Task) -> Result<Task> {
    self.save(task)
  }

  pub fn update(&self, mut task: Task) -> Result<Task> {
    task.set_updated_at(Utc::now());
    self.save(task)
  }

  fn save(&self, task: Task) -> Result<Task> {
    let mut statement = self.connection.prepare(UPSERT_SQL)?;
    statement.execute(task.to_sql_params())?;
    self.by_pk(task.id())
  }
}

const UPSERT_SQL: &str = r"
  INSERT INTO tasks (
    id,
    external_id,
    project_id,
    title,
    description,
    priority,
    workflow_status,
    estimation,
    resolution,
    due_at,
    completed_at,
    created_at,
    updated_at
  )
  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
  ON CONFLICT (id) DO UPDATE SET
    external_id = excluded.external_id,
    project_id = excluded.project_id,
    title = excluded.title,
    description = excluded.description,
    priority = excluded.priority,
    workflow_status = excluded.workflow_status,
    estimation = excluded.estimation,
    resolution = excluded.resolution,
    due_at = excluded.due_at,
    completed_at = excluded.completed_at,
    updated_at = excluded.updated_at
";
