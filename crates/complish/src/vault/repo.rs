use std::path::PathBuf;

use eyre::Result;
use rusqlite::Connection;

use crate::{list::repo::Repo as ListRepo, task::repo::Repo as TaskRepo};

pub struct Repo {
  connection: Connection,
}

impl Repo {
  pub fn new(vault_path: PathBuf) -> Result<Self> {
    let connection = Connection::open(vault_path)?;

    Ok(Self {
      connection,
    })
  }

  pub fn list(&self) -> ListRepo<'_> {
    ListRepo::new(&self.connection)
  }

  pub fn task(&self) -> TaskRepo<'_> {
    TaskRepo::new(&self.connection)
  }
}

#[cfg(test)]
mod tests {
  use temp_dir::TempDir;

  use super::*;
  use crate::vault::migrations;

  mod list {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_provides_the_list_repo() {
      let temp_dir = TempDir::new().unwrap();
      let vault_path = temp_dir.path().join("vault");
      migrations::run(&vault_path).unwrap();

      let repo = Repo::new(vault_path).unwrap();
      let today = repo.list().by_name("today").unwrap();

      assert_eq!(today.name(), "Today");
    }
  }

  mod task {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::task::Status;

    #[test]
    fn it_provides_the_task_repo() {
      let temp_dir = TempDir::new().unwrap();
      let vault_path = temp_dir.path().join("vault");
      migrations::run(&vault_path).unwrap();

      let connection = Connection::open(&vault_path).unwrap();
      connection
        .execute(
          "INSERT INTO tasks (list_id, subject, status) VALUES (?1, ?2, ?3)",
          (3, "a test task", Status::Todo),
        )
        .unwrap();

      let repo = Repo::new(vault_path).unwrap();
      let task = repo.task().by_pk(1).unwrap();

      assert_eq!(task.subject(), "a test task");
    }
  }
}
