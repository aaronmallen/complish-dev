use eyre::Result;
use rusqlite::types::ToSql;

use super::constructor::*;
use crate::Task;

impl<'a> Repo<'a> {
  pub fn all(&self) -> Result<Vec<Task>> {
    self.all_by_sql(ALL_SQL)
  }

  pub fn all_active(&self) -> Result<Vec<Task>> {
    self.all_by_sql(ALL_ACTIVE_SQL)
  }

  pub fn by_external_id(&self, id: impl Into<String>) -> Result<Task> {
    self.by_sql(BY_EXTERNAL_ID_SQL, [&id.into()])
  }

  pub fn by_pk(&self, id: impl Into<String>) -> Result<Task> {
    self.by_sql(BY_PK_SQL, [&id.into()])
  }

  pub fn by_sequence_id(&self, sequence_id: u32) -> Result<Task> {
    self.by_sql(BY_SEQUENCE_ID_SQL, [&sequence_id.to_string()])
  }

  fn all_by_sql(&self, sql: &str) -> Result<Vec<Task>> {
    let mut statement = self.connection.prepare(sql)?;
    statement
      .query_map([], |row| Task::try_from(row))?
      .map(|r| r.map_err(Into::into).and_then(|t| self.hydrate(t)))
      .collect()
  }

  fn by_sql(&self, sql: &str, params: [&dyn ToSql; 1]) -> Result<Task> {
    let mut statement = self.connection.prepare(sql)?;
    let task = statement.query_row(params, |row| Task::try_from(row))?;

    self.hydrate(task)
  }
}

const ALL_SQL: &str = "SELECT * FROM tasks";

const ALL_ACTIVE_SQL: &str = r"
  SELECT *
  FROM tasks
  WHERE workflow_status != 'done'
";

const BY_EXTERNAL_ID_SQL: &str = "SELECT * FROM tasks WHERE external_id = ?1";

const BY_PK_SQL: &str = "SELECT * FROM tasks WHERE id = ?1";

const BY_SEQUENCE_ID_SQL: &str = "SELECT * FROM tasks WHERE sequence_id = ?1";
