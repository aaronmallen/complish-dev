use eyre::Result;
use rusqlite::{Connection, Result as SqliteResult};

use crate::{
  List,
  task::{Task, repo::Repo as TaskRepo},
};

pub struct Repo<'a> {
  connection: &'a Connection,
}

impl<'a> Repo<'a> {
  pub const SELECT_BY_NAME_SQL: &'static str = r"
    SELECT * FROM lists WHERE LOWER(name) = LOWER(?1)
  ";
  pub const SELECT_BY_PK_SQL: &'static str = r"
    SELECT * FROM lists WHERE id = ?1
  ";

  pub fn new(connection: &'a Connection) -> Self {
    Self {
      connection,
    }
  }

  pub fn by_name(&self, name: impl Into<String>) -> Result<List> {
    let mut statement = self.connection.prepare(Self::SELECT_BY_NAME_SQL)?;

    let mut list = statement.query_row([name.into()], |row| List::try_from(row))?;
    list.tasks = self.get_tasks_for_list(list.id)?;
    Ok(list)
  }

  pub fn by_pk(&self, id: u32) -> Result<List> {
    let mut statement = self.connection.prepare(Self::SELECT_BY_PK_SQL)?;

    let mut list = statement.query_row([id], |row| List::try_from(row))?;
    list.tasks = self.get_tasks_for_list(list.id)?;
    Ok(list)
  }

  fn get_tasks_for_list(&self, list_id: u32) -> SqliteResult<Vec<Task>> {
    let mut statement = self.connection.prepare(TaskRepo::SELECT_BY_PK_SQL)?;

    statement
      .query_map([list_id], |row| Task::try_from(row))?
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use rusqlite::Connection;
  use temp_dir::TempDir;

  use crate::vault::migrations;

  fn get_test_connection() -> Connection {
    let temp_dir = TempDir::new().unwrap();
    let vault_path = temp_dir.path().join("vault");
    migrations::run(&vault_path).unwrap();
    Connection::open(vault_path).unwrap()
  }

  mod by_name {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::list::repo::Repo;

    #[test]
    fn it_returns_the_list_by_name() {
      let connection = get_test_connection();
      let repo = Repo::new(&connection);
      let today = repo.by_name("today").unwrap();
      let next = repo.by_name("next").unwrap();
      let someday = repo.by_name("someday").unwrap();

      assert_eq!(today.name, "Today");
      assert_eq!(next.name, "Next");
      assert_eq!(someday.name, "Someday");
    }

    #[test]
    fn it_returns_an_error_if_the_list_does_not_exist() {
      let connection = get_test_connection();
      let repo = Repo::new(&connection);
      let result = repo.by_name("does not exist");

      assert!(result.is_err());
    }
  }

  mod by_pk {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::list::repo::Repo;

    #[test]
    fn it_returns_the_list_by_pk() {
      let connection = get_test_connection();
      let repo = Repo::new(&connection);
      let today = repo.by_pk(1).unwrap();
      let next = repo.by_pk(2).unwrap();
      let someday = repo.by_pk(3).unwrap();

      assert_eq!(today.name, "Today");
      assert_eq!(next.name, "Next");
      assert_eq!(someday.name, "Someday");
    }

    #[test]
    fn it_returns_an_error_if_the_list_does_not_exist() {
      let connection = get_test_connection();
      let repo = Repo::new(&connection);
      let result = repo.by_pk(4);

      assert!(result.is_err());
    }
  }
}
