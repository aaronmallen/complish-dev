use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

use super::update_status::UpdateStatus;

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize, Setters)]
pub struct Update {
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[getset(get = "pub", set = "pub")]
  description: Option<String>,
  #[get = "pub"]
  id: String,
  #[get = "pub"]
  project_id: String,
  #[getset(get = "pub", set = "pub")]
  status: UpdateStatus,
  #[getset(get = "pub", set = "pub")]
  updated_at: DateTime<Utc>,
}

impl Update {
  pub fn new(project_id: impl Into<String>, status: UpdateStatus) -> Self {
    let now = Utc::now();

    Self {
      created_at: now,
      description: None,
      id: cuid2::create_id(),
      project_id: project_id.into(),
      status,
      updated_at: now,
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 6] {
    [
      &self.id,
      &self.project_id,
      &self.description,
      &self.status,
      &self.created_at,
      &self.updated_at,
    ]
  }

  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }
}

impl TryFrom<&Row<'_>> for Update {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      created_at: row.get("created_at")?,
      description: row.get("description")?,
      id: row.get("id")?,
      project_id: row.get("project_id")?,
      status: row.get("status")?,
      updated_at: row.get("updated_at")?,
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod with_description {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_applies_the_description() {
      let description = "This is a description";
      let update =
        Update::new(cuid2::create_id(), UpdateStatus::OnTrack).with_description(description);

      assert_eq!(
        update.description().clone().unwrap(),
        description.to_string()
      );
    }
  }

  mod to_sql_params {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;
    use crate::project::Project;

    #[test]
    fn it_returns_the_update_fields_as_sql_params() {
      let connection = Connection::open_in_memory().unwrap();
      crate::db::migrations::run_with_connection(&connection).unwrap();

      let project = Project::new("test");
      connection
        .execute(
          r"
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
      ",
          project.to_sql_params(),
        )
        .unwrap();

      let result = connection.execute(
        r"
          INSERT INTO project_updates (id, project_id, description, status, created_at, updated_at)
          VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ",
        Update::new(project.id(), UpdateStatus::OnTrack).to_sql_params(),
      );

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), 1);
    }
  }

  mod try_from_row {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_returns_an_update() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection
        .prepare(
          r"
          SELECT
            'popmfy0xiz8ykp3urgojxtma' AS id,
            'k4dp8lin5igq5t0atazbf5xw' AS project_id,
            'test' AS description,
            'on track' AS status,
            datetime('now', 'utc') AS created_at,
            datetime('now', 'utc') AS updated_at
        ",
        )
        .unwrap();

      let update = statement
        .query_row([], |row| Update::try_from(row))
        .unwrap();

      assert_eq!(update.id(), "popmfy0xiz8ykp3urgojxtma");
      assert_eq!(update.project_id(), "k4dp8lin5igq5t0atazbf5xw");
      assert_eq!(update.description(), &Some("test".to_string()));
      assert_eq!(update.status(), &UpdateStatus::OnTrack);
    }
  }
}
