use std::{
  cmp::Ordering,
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use eyre::{Error, Result, eyre};
use rusqlite::{
  ToSql,
  types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
  Done,
  InProgress,
  #[default]
  Todo,
}

impl Display for Status {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Done => write!(f, "done"),
      Self::InProgress => write!(f, "in progress"),
      Self::Todo => write!(f, "todo"),
    }
  }
}

impl FromSql for Status {
  fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
    let s = value.as_str()?;
    s.parse().map_err(|_| FromSqlError::InvalidType)
  }
}

impl FromStr for Status {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s.to_lowercase().as_str() {
      "done" => Ok(Self::Done),
      "in progress" | "in_progress" => Ok(Self::InProgress),
      "todo" => Ok(Self::Todo),
      _ => Err(eyre!(format!("invalid task status: {}", s))),
    }
  }
}

impl Ord for Status {
  fn cmp(&self, other: &Self) -> Ordering {
    let self_ord = match self {
      Self::Done => 2,
      Self::InProgress => 1,
      Self::Todo => 0,
    };

    let other_ord = match other {
      Self::Done => 2,
      Self::InProgress => 1,
      Self::Todo => 0,
    };

    self_ord.cmp(&other_ord)
  }
}

impl PartialOrd for Status {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl ToSql for Status {
  fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
    Ok(ToSqlOutput::from(self.to_string()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_displays_correctly() {
      assert_eq!(Status::Done.to_string(), "done");
      assert_eq!(Status::InProgress.to_string(), "in progress");
      assert_eq!(Status::Todo.to_string(), "todo");
    }
  }

  mod from_str {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses_correctly() {
      assert_eq!(Status::from_str("done").unwrap(), Status::Done);
      assert_eq!(Status::from_str("in_progress").unwrap(), Status::InProgress);
      assert_eq!(Status::from_str("in progress").unwrap(), Status::InProgress);
      assert_eq!(Status::from_str("todo").unwrap(), Status::Todo);
    }

    #[test]
    fn it_errors_on_invalid_input() {
      assert!(Status::from_str("invalid").is_err());
    }

    #[test]
    fn it_parses_case_insensitively() {
      for str in &["TODO", "Todo", "ToDo"] {
        assert_eq!(Status::from_str(str).unwrap(), Status::Todo);
      }
    }
  }

  mod ordering {
    use super::*;

    #[test]
    fn it_orders_correctly() {
      assert!(Status::Todo < Status::InProgress);
      assert!(Status::InProgress < Status::Done);
    }
  }
}
