use chrono::{DateTime, Utc};
use getset::Getters;
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

use crate::entities::task::relationship_type::RelationshipType;

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct Relationship {
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get = "pub"]
  kind: RelationshipType,
  #[get = "pub"]
  source_id: u32,
  #[get = "pub"]
  target_id: u32,
  #[get = "pub"]
  updated_at: DateTime<Utc>,
}

impl Relationship {
  pub fn new(source_id: u32, target_id: u32, kind: RelationshipType) -> Self {
    let now = Utc::now();

    Self {
      created_at: now,
      kind,
      source_id,
      target_id,
      updated_at: now,
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 5] {
    [
      &self.created_at,
      &self.kind,
      &self.source_id,
      &self.target_id,
      &self.updated_at,
    ]
  }
}

impl TryFrom<&Row<'_>> for Relationship {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      created_at: row.get("created_at")?,
      kind: row.get("kind")?,
      source_id: row.get("source_id")?,
      target_id: row.get("target_id")?,
      updated_at: row.get("updated_at")?,
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

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
        CREATE TABLE task_relationships (
          source_id INTEGER NOT NULL,
          target_id INTEGER NOT NULL,
          kind TEXT NOT NULL,
          created_at TIMESTAMP NOT NULL,
          updated_at TIMESTAMP NOT NULL
        )
      ",
          [],
        )
        .unwrap();

      let relationship = Relationship::new(1, 2, RelationshipType::Blocks);

      let result = connection.execute(
        r"
        INSERT INTO task_relationships (source_id, target_id, kind, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
      ",
        relationship.to_sql_params(),
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
    fn it_converts_a_row_into_a_relationship() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection
        .prepare(
          r"
        SELECT
          1 AS source_id,
          2 AS target_id,
          'blocks' AS kind,
          datetime('now', 'utc') AS created_at,
          datetime('now', 'utc') AS updated_at
      ",
        )
        .unwrap();
      let relationship = statement
        .query_row([], |row| Relationship::try_from(row))
        .unwrap();

      assert_eq!(relationship.source_id(), &1);
      assert_eq!(relationship.target_id(), &2);
      assert_eq!(relationship.kind(), &RelationshipType::Blocks);
      assert!(relationship.created_at().timestamp() > 0);
      assert!(relationship.updated_at().timestamp() > 0);
    }
  }
}
