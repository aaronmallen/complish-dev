use eyre::Result;
use rusqlite::Connection;

use crate::task::Status;

#[derive(Clone, Debug)]
pub struct Creator<'a> {
  connection: &'a Connection,
  description: Option<String>,
  list_id: u32,
  subject: String,
}

impl<'a> Creator<'a> {
  pub const INSERT_INTO_TASKS_SQL: &'static str = r"
    INSERT INTO tasks (list_id, subject, description, status)
    VALUES
      (?1, ?2, ?3, ?4)
  ";

  pub fn new(connection: &'a Connection, subject: impl Into<String>) -> Self {
    Self {
      connection,
      description: None,
      list_id: 3,
      subject: subject.into(),
    }
  }

  pub fn create(&self) -> Result<u32> {
    self.connection.execute(
      Self::INSERT_INTO_TASKS_SQL,
      (
        self.list_id,
        self.subject.clone(),
        self.description.clone(),
        Status::default(),
      ),
    )?;

    Ok(u32::try_from(self.connection.last_insert_rowid())?)
  }

  pub fn in_list(mut self, list_id: u32) -> Result<Self> {
    if !(1..=3).contains(&list_id) {
      return Err(eyre::eyre!("Invalid list id {}", list_id));
    }

    self.list_id = list_id;
    Ok(self)
  }

  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
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

  mod create {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::task::creator::Creator;

    #[test]
    fn it_returns_the_id_of_the_created_task() {
      let (_temp_dir, connection) = get_test_connection();
      let creator = Creator::new(&connection, "a test task");

      assert_eq!(creator.create().unwrap(), 1);
    }
  }

  mod in_list {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::task::{creator::Creator, repo::Repo};

    #[test]
    fn it_assigns_the_correct_list_to_the_task() {
      let (_temp_dir, connection) = get_test_connection();
      let task_id = Creator::new(&connection, "a test task")
        .in_list(2)
        .unwrap()
        .create()
        .unwrap();
      let task = Repo::new(&connection).by_pk(task_id).unwrap();

      assert_eq!(task.list_id, 2);
    }

    #[test]
    fn it_returns_an_error_if_the_list_id_is_invalid() {
      let (_temp_dir, connection) = get_test_connection();
      let creator = Creator::new(&connection, "a test task");

      assert!(creator.clone().in_list(0).is_err());
      assert!(creator.clone().in_list(4).is_err());
    }
  }

  mod with_description {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::task::{creator::Creator, repo::Repo};

    #[test]
    fn it_assigns_a_description_to_the_task() {
      let (_temp_dir, connection) = get_test_connection();
      let task_id = Creator::new(&connection, "a test task")
        .with_description("a test description")
        .create()
        .unwrap();
      let task = Repo::new(&connection).by_pk(task_id).unwrap();

      assert_eq!(task.description, Some("a test description".to_string()));
    }
  }
}
