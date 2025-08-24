use rusqlite::{Connection, Result as SqliteResult};

use crate::task::Task;

pub struct Repo<'a> {
  connection: &'a Connection,
}

impl<'a> Repo<'a> {
  pub const SELECT_BY_PK_SQL: &'static str = r"
    SELECT * FROM tasks WHERE id = ?1
  ";

  pub fn new(connection: &'a Connection) -> Self {
    Self {
      connection,
    }
  }

  pub fn by_pk(&self, id: u32) -> SqliteResult<Task> {
    let mut statement = self.connection.prepare(Self::SELECT_BY_PK_SQL)?;

    statement.query_row([id], |row| Task::try_from(row))
  }
}

#[cfg(test)]
mod tests {
  use rusqlite::Connection;
  use temp_dir::TempDir;

  use crate::vault::migrations;

  fn get_test_connection() -> (TempDir, Connection) {
    let temp_dir = TempDir::new().unwrap();
    let vault_path = temp_dir.path().join("vault");
    migrations::run(&vault_path).unwrap();
    (temp_dir, Connection::open(vault_path).unwrap())
  }

  mod by_pk {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::task::{Status, repo::Repo};

    #[test]
    fn it_returns_the_task_by_pk() {
      let (_temp_dir, connection) = get_test_connection();
      connection
        .execute(
          "INSERT INTO tasks (list_id, subject, status) VALUES (?1, ?2, ?3)",
          (3, "a test task", Status::Todo),
        )
        .unwrap();
      let repo = Repo::new(&connection);
      let task = repo.by_pk(1).unwrap();

      assert_eq!(task.subject, "a test task");
    }
  }
}
