use rusqlite::{Connection, Result as SqliteResult};

use crate::{
  list::Name as ListName,
  task::{Status, Task, creator::Creator},
};

pub struct Repo<'a> {
  connection: &'a Connection,
}

impl<'a> Repo<'a> {
  pub const SELECT_BY_PK_SQL: &'static str = r"
    SELECT * FROM tasks WHERE id = ?1
  ";
  pub const SET_LIST_ID_SQL: &'static str = r"
    UPDATE tasks SET list_id = ?2 WHERE id = ?1
  ";
  pub const SET_STATUS_SQL: &'static str = r"
    UPDATE tasks SET status = ?2 WHERE id = ?1
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

  pub fn create(&self, subject: impl Into<String>) -> Creator<'_> {
    Creator::new(self.connection, subject)
  }

  pub fn move_to_in_progress(&self, id: u32) -> SqliteResult<Task> {
    self
      .connection
      .execute(Self::SET_STATUS_SQL, (id, Status::InProgress))?;
    self.by_pk(id)
  }

  pub fn move_to_list(&self, id: u32, list_name: ListName) -> SqliteResult<Task> {
    let list_id = list_name.id();
    self
      .connection
      .execute(Self::SET_LIST_ID_SQL, (id, list_id))?;
    self.by_pk(id)
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

  mod create {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::task::repo::Repo;

    #[test]
    fn it_returns_a_creator() {
      let (_temp_dir, connection) = get_test_connection();
      let repo = Repo::new(&connection);
      let creator = repo.create("a test task");
      let task_id = creator.create().unwrap();

      assert_eq!(task_id, 1);
    }
  }

  mod move_to_in_progress {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::task::{Status, repo::Repo};

    #[test]
    fn it_move_the_task_to_in_progress_status() {
      let (_temp_dir, connection) = get_test_connection();
      let repo = Repo::new(&connection);
      let task_id = repo.create("a test task").create().unwrap();
      let before_task = repo.by_pk(task_id).unwrap();

      assert_eq!(before_task.status, Status::Todo);

      let after_task = repo.move_to_in_progress(task_id).unwrap();

      assert_eq!(after_task.status, Status::InProgress);
    }
  }

  mod move_to_list {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{list::Name as ListName, task::repo::Repo};

    #[test]
    fn it_move_the_task_to_a_new_list() {
      let (_temp_dir, connection) = get_test_connection();
      let repo = Repo::new(&connection);
      let task_id = repo.create("a test task").create().unwrap();
      let before_task = repo.by_pk(task_id).unwrap();

      assert_eq!(before_task.list_id, ListName::Someday.id());

      let after_task = repo.move_to_list(task_id, ListName::Today).unwrap();

      assert_eq!(after_task.list_id, ListName::Today.id());
    }
  }
}
