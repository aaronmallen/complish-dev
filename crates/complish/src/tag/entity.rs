use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize, Setters)]
pub struct Tag {
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", set = "pub")]
  label: String,
  #[getset(get = "pub", set = "pub")]
  updated_at: DateTime<Utc>,
}

impl Tag {
  pub fn new(label: impl Into<String>) -> Self {
    let now = Utc::now();

    Self {
      created_at: now,
      id: cuid2::create_id(),
      label: label.into(),
      updated_at: now,
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 4] {
    [&self.id, &self.label, &self.created_at, &self.updated_at]
  }
}

impl TryFrom<&Row<'_>> for Tag {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      created_at: row.get("created_at")?,
      id: row.get("id")?,
      label: row.get("label")?,
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
    fn it_returns_the_tag_fields_as_sql_params() {
      let connection = Connection::open_in_memory().unwrap();
      crate::db::migrations::run_with_connection(&connection).unwrap();

      let tag = Tag::new("test");

      let result = connection.execute(
        r"
          INSERT INTO tags (id, label, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)
        ",
        tag.to_sql_params(),
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
    fn it_returns_a_tag() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection
        .prepare(
          r"
          SELECT
            'popmfy0xiz8ykp3urgojxtma' AS id,
            'test' AS label,
            datetime('now', 'utc') AS created_at,
            datetime('now', 'utc') AS updated_at
        ",
        )
        .unwrap();
      let tag = statement.query_row([], |row| Tag::try_from(row)).unwrap();

      assert_eq!(tag.id(), "popmfy0xiz8ykp3urgojxtma");
      assert_eq!(tag.label(), "test");
    }
  }
}
