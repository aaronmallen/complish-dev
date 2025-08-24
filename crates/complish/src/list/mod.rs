pub(crate) mod repo;

use chrono::{DateTime, Utc};
use getset::Getters;
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row};
use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct List {
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get = "pub"]
  id: u32,
  #[get = "pub"]
  name: String,
  #[get = "pub"]
  tasks: Vec<Task>,
  #[get = "pub"]
  updated_at: DateTime<Utc>,
}

impl TryFrom<&Row<'_>> for List {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(List {
      created_at: row.get("created_at")?,
      id: row.get("id")?,
      name: row.get("name")?,
      tasks: Vec::new(),
      updated_at: row.get("updated_at")?,
    })
  }
}

#[cfg(test)]
mod tests {
  use rusqlite::{Connection, Result as SqliteResult};
  use temp_dir::TempDir;

  use super::*;
  use crate::vault::migrations;

  fn get_connection() -> Connection {
    let temp_dir = TempDir::new().unwrap();
    let vault_path = temp_dir.path().join("vault");
    migrations::run(&vault_path).unwrap();
    Connection::open(vault_path).unwrap()
  }

  mod try_from {
    use super::*;

    #[test]
    fn it_converts_row_to_list() {
      let connection = get_connection();
      let mut statement = connection
        .prepare("SELECT * FROM lists WHERE id = 1")
        .unwrap();
      let list: List = statement.query_row([], |row| List::try_from(row)).unwrap();

      assert_eq!(list.name(), "Today");
    }

    #[test]
    fn it_handles_missing_columns() {
      let connection = get_connection();
      let mut statement = connection.prepare("SELECT 1 as dummy").unwrap();
      let result: SqliteResult<List> = statement.query_row([], |row| List::try_from(row));

      assert!(result.is_err());
    }
  }
}
