use eyre::Result;
use rusqlite::types::ToSql;

use super::constructor::*;
use crate::Project;

impl<'a> Repo<'a> {
  pub fn all(&self) -> Result<Vec<Project>> {
    self.all_by_sql(ALL_SQL)
  }

  pub fn all_active(&self) -> Result<Vec<Project>> {
    self.all_by_sql(ALL_ACTIVE_SQL)
  }

  pub fn by_key(&self, key: impl Into<String>) -> Result<Project> {
    self.by_sql(BY_KEY_SQL, [&key.into()])
  }

  pub fn by_pk(&self, id: impl Into<String>) -> Result<Project> {
    self.by_sql(BY_PK_SQL, [&id.into()])
  }

  fn all_by_sql(&self, sql: &str) -> Result<Vec<Project>> {
    let mut statement = self.connection.prepare(sql)?;
    statement
      .query_map([], |row| Project::try_from(row))?
      .map(|r| r.map_err(Into::into).and_then(|p| self.hydrate(p)))
      .collect()
  }

  fn by_sql(&self, sql: &str, params: [&dyn ToSql; 1]) -> Result<Project> {
    let mut statement = self.connection.prepare(sql)?;
    let project = statement.query_row(params, |row| Project::try_from(row))?;

    self.hydrate(project)
  }
}

const ALL_SQL: &str = "SELECT * FROM projects";

const ALL_ACTIVE_SQL: &str = "SELECT * FROM projects WHERE completed_at IS NULL";

const BY_KEY_SQL: &str = "SELECT * FROM projects WHERE key = ?1";

const BY_PK_SQL: &str = "SELECT * FROM projects WHERE id = ?1";
