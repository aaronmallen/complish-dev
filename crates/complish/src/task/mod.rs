mod creator;
pub(crate) mod repo;
mod status;

use chrono::{DateTime, Utc};
use getset::{CloneGetters, Getters};
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row};
use serde::{Deserialize, Serialize};
pub use status::Status;

#[derive(Clone, CloneGetters, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct Task {
  #[get_clone = "pub"]
  completed_at: Option<DateTime<Utc>>,
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get_clone = "pub"]
  description: Option<String>,
  #[get = "pub"]
  id: u32,
  #[get = "pub"]
  list_id: u32,
  #[get = "pub"]
  status: Status,
  #[get = "pub"]
  subject: String,
  #[get = "pub"]
  updated_at: DateTime<Utc>,
}

impl TryFrom<&Row<'_>> for Task {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Task {
      completed_at: row.get("completed_at")?,
      created_at: row.get("created_at")?,
      description: row.get("description")?,
      id: row.get("id")?,
      list_id: row.get("list_id")?,
      status: row.get("status")?,
      subject: row.get("subject")?,
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

  fn get_connection() -> (TempDir, Connection) {
    let temp_dir = TempDir::new().unwrap();
    let vault_path = temp_dir.path().join("vault");
    migrations::run(&vault_path).unwrap();
    let connection = Connection::open(vault_path).unwrap();
    (temp_dir, connection)
  }

  mod try_from {
    use super::*;

    #[test]
    fn it_converts_row_to_list() {
      let (_temp_dir, connection) = get_connection();
      connection
        .execute(
          "INSERT INTO tasks (list_id, subject, status) VALUES (?1, ?2, ?3)",
          (3, "A test task".to_string(), Status::Todo),
        )
        .unwrap();

      let mut statement = connection
        .prepare("SELECT * FROM tasks WHERE id = ?1")
        .unwrap();

      let task = statement.query_row([1], |row| Task::try_from(row)).unwrap();
      assert_eq!(task.subject, "A test task");
    }

    #[test]
    fn it_handles_missing_columns() {
      let (_, connection) = get_connection();
      let mut stmt = connection.prepare("SELECT 1 as dummy").unwrap();
      let result: SqliteResult<Task> = stmt.query_row([], |row| Task::try_from(row));

      assert!(result.is_err());
    }
  }
}
