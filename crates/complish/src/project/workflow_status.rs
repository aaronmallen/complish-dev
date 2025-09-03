use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use eyre::{Error, Result, eyre};
use rusqlite::{
  Result as SqliteResult,
  types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
};
use serde::{Deserialize, Serialize};

use crate::macros::impl_sql_string_traits;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
  Done,
  InProgress,
  Planned,
  #[default]
  Todo,
}

impl_sql_string_traits!(WorkflowStatus);

impl Display for WorkflowStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Done => write!(f, "done"),
      Self::InProgress => write!(f, "in progress"),
      Self::Planned => write!(f, "planned"),
      Self::Todo => write!(f, "todo"),
    }
  }
}

impl FromStr for WorkflowStatus {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s.to_lowercase().as_str() {
      "done" => Ok(Self::Done),
      "in progress" | "in_progress" => Ok(Self::InProgress),
      "planned" => Ok(Self::Planned),
      "todo" => Ok(Self::Todo),
      _ => Err(eyre!("Invalid workflow status: {}", s)),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_displays() {
      assert_eq!(WorkflowStatus::Done.to_string(), "done");
      assert_eq!(WorkflowStatus::InProgress.to_string(), "in progress");
      assert_eq!(WorkflowStatus::Planned.to_string(), "planned");
      assert_eq!(WorkflowStatus::Todo.to_string(), "todo");
    }
  }

  mod from_sql {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_parses() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection.prepare("SELECT ?1").unwrap();
      let done: WorkflowStatus = statement.query_row(["done"], |row| row.get(0)).unwrap();
      let in_progress: WorkflowStatus = statement
        .query_row(["in progress"], |row| row.get(0))
        .unwrap();
      let planned: WorkflowStatus = statement.query_row(["planned"], |row| row.get(0)).unwrap();
      let todo: WorkflowStatus = statement.query_row(["todo"], |row| row.get(0)).unwrap();

      assert_eq!(done, WorkflowStatus::Done);
      assert_eq!(in_progress, WorkflowStatus::InProgress);
      assert_eq!(planned, WorkflowStatus::Planned);
      assert_eq!(todo, WorkflowStatus::Todo);
    }
  }

  mod from_str {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses() {
      assert_eq!(
        WorkflowStatus::from_str("done").unwrap(),
        WorkflowStatus::Done
      );

      for str in &["in progress", "in_progress"] {
        assert_eq!(
          WorkflowStatus::from_str(str).unwrap(),
          WorkflowStatus::InProgress
        );
      }

      assert_eq!(
        WorkflowStatus::from_str("planned").unwrap(),
        WorkflowStatus::Planned
      );
      assert_eq!(
        WorkflowStatus::from_str("todo").unwrap(),
        WorkflowStatus::Todo
      );
    }

    #[test]
    fn it_errors_on_invalid_input() {
      assert!(WorkflowStatus::from_str("invalid").is_err());
    }

    #[test]
    fn it_parses_case_insensitively() {
      for str in ["DONE", "Done", "DoNe"] {
        assert_eq!(WorkflowStatus::from_str(str).unwrap(), WorkflowStatus::Done);
      }
    }
  }

  mod to_sql {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_serializes() {
      let connection = Connection::open_in_memory().unwrap();
      connection
        .execute("CREATE TABLE test (status TEXT)", [])
        .unwrap();

      for status in [
        WorkflowStatus::Done,
        WorkflowStatus::InProgress,
        WorkflowStatus::Planned,
        WorkflowStatus::Todo,
      ] {
        connection
          .execute("INSERT INTO test (status) VALUES (?1)", [status])
          .unwrap();
      }

      let mut statement = connection
        .prepare("SELECT * FROM test where status = ?1")
        .unwrap();

      let done: WorkflowStatus = statement
        .query_row([WorkflowStatus::Done], |row| row.get(0))
        .unwrap();
      let in_progress: WorkflowStatus = statement
        .query_row([WorkflowStatus::InProgress], |row| row.get(0))
        .unwrap();
      let planned: WorkflowStatus = statement
        .query_row([WorkflowStatus::Planned], |row| row.get(0))
        .unwrap();
      let todo: WorkflowStatus = statement
        .query_row([WorkflowStatus::Todo], |row| row.get(0))
        .unwrap();

      assert_eq!(done, WorkflowStatus::Done);
      assert_eq!(in_progress, WorkflowStatus::InProgress);
      assert_eq!(planned, WorkflowStatus::Planned);
      assert_eq!(todo, WorkflowStatus::Todo);
    }
  }
}
