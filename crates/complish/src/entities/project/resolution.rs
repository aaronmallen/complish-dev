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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[serde(rename_all = "snake_case")]
pub enum Resolution {
  Canceled,
  Completed,
}

impl_sql_string_traits!(Resolution);

impl Display for Resolution {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Canceled => write!(f, "canceled"),
      Self::Completed => write!(f, "completed"),
    }
  }
}

impl FromStr for Resolution {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s.to_lowercase().as_str() {
      "canceled" => Ok(Self::Canceled),
      "completed" => Ok(Self::Completed),
      _ => Err(eyre!("invalid resolution: {}", s)),
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
      assert_eq!(Resolution::Canceled.to_string(), "canceled");
      assert_eq!(Resolution::Completed.to_string(), "completed");
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
      let canceled: Resolution = statement.query_row(["canceled"], |row| row.get(0)).unwrap();
      let completed: Resolution = statement
        .query_row(["completed"], |row| row.get(0))
        .unwrap();

      assert_eq!(canceled, Resolution::Canceled);
      assert_eq!(completed, Resolution::Completed);
    }
  }

  mod from_str {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses() {
      assert_eq!(
        Resolution::from_str("canceled").unwrap(),
        Resolution::Canceled
      );
      assert_eq!(
        Resolution::from_str("completed").unwrap(),
        Resolution::Completed
      );
    }

    #[test]
    fn it_errors_on_invalid_input() {
      assert!(Resolution::from_str("invalid").is_err());
    }

    #[test]
    fn it_parses_case_insensitively() {
      for str in ["CANCELED", "Canceled", "CaNcElEd"] {
        assert_eq!(Resolution::from_str(str).unwrap(), Resolution::Canceled);
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
        .execute("CREATE TABLE test (resolution TEXT)", [])
        .unwrap();

      for resolution in [Resolution::Canceled, Resolution::Completed] {
        connection
          .execute("INSERT INTO test (resolution) VALUES (?1)", [resolution])
          .unwrap();
      }

      let mut statement = connection
        .prepare("SELECT * FROM test where resolution = ?1")
        .unwrap();
      let canceled: Resolution = statement
        .query_row([Resolution::Canceled], |row| row.get(0))
        .unwrap();
      let completed: Resolution = statement
        .query_row([Resolution::Completed], |row| row.get(0))
        .unwrap();

      assert_eq!(canceled, Resolution::Canceled);
      assert_eq!(completed, Resolution::Completed);
    }
  }
}
