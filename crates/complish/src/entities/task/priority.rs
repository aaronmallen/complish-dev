use std::{
  cmp::Ordering,
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use eyre::{Error, Result, eyre};
use rusqlite::{
  Result as SqliteResult, ToSql,
  types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
};
use serde::{Deserialize, Serialize};

use crate::macros::impl_sql_string_traits;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[serde(rename_all = "snake_case")]
pub enum Priority {
  Critical = 0,
  High = 1,
  #[default]
  Medium = 2,
  Low = 3,
  Lowest = 4,
}

impl_sql_string_traits!(Priority);

impl Priority {
  pub fn as_named_str(&self) -> &str {
    match self {
      Priority::Critical => "critical",
      Priority::High => "high",
      Priority::Medium => "medium",
      Priority::Low => "low",
      Priority::Lowest => "lowest",
    }
  }
}

impl Display for Priority {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Priority::Critical => write!(f, "p0"),
      Priority::High => write!(f, "p1"),
      Priority::Medium => write!(f, "p2"),
      Priority::Low => write!(f, "p3"),
      Priority::Lowest => write!(f, "p4"),
    }
  }
}

impl FromStr for Priority {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s.to_lowercase().as_str() {
      "critical" | "p0" => Ok(Priority::Critical),
      "high" | "p1" => Ok(Priority::High),
      "medium" | "p2" => Ok(Priority::Medium),
      "low" | "p3" => Ok(Priority::Low),
      "lowest" | "p4" => Ok(Priority::Lowest),
      _ => Err(eyre!("invalid priority: {}", s)),
    }
  }
}

impl Ord for Priority {
  fn cmp(&self, other: &Self) -> Ordering {
    (other.clone() as u8).cmp(&(self.clone() as u8))
  }
}

impl PartialOrd for Priority {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod named_str {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_the_named_string() {
      assert_eq!(Priority::Critical.as_named_str(), "critical");
      assert_eq!(Priority::High.as_named_str(), "high");
      assert_eq!(Priority::Medium.as_named_str(), "medium");
      assert_eq!(Priority::Low.as_named_str(), "low");
      assert_eq!(Priority::Lowest.as_named_str(), "lowest");
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_displays() {
      assert_eq!(Priority::Critical.to_string(), "p0");
      assert_eq!(Priority::High.to_string(), "p1");
      assert_eq!(Priority::Medium.to_string(), "p2");
      assert_eq!(Priority::Low.to_string(), "p3");
      assert_eq!(Priority::Lowest.to_string(), "p4");
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
      let critical: Priority = statement.query_row(["p0"], |row| row.get(0)).unwrap();
      let high: Priority = statement.query_row(["p1"], |row| row.get(0)).unwrap();
      let medium: Priority = statement.query_row(["p2"], |row| row.get(0)).unwrap();
      let low: Priority = statement.query_row(["p3"], |row| row.get(0)).unwrap();
      let lowest: Priority = statement.query_row(["p4"], |row| row.get(0)).unwrap();

      assert_eq!(critical, Priority::Critical);
      assert_eq!(high, Priority::High);
      assert_eq!(medium, Priority::Medium);
      assert_eq!(low, Priority::Low);
      assert_eq!(lowest, Priority::Lowest);
    }
  }

  mod from_str {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::entities::project::resolution::Resolution;

    #[test]
    fn it_parses() {
      assert_eq!(Priority::from_str("critical").unwrap(), Priority::Critical);
      assert_eq!(Priority::from_str("p0").unwrap(), Priority::Critical);
      assert_eq!(Priority::from_str("high").unwrap(), Priority::High);
      assert_eq!(Priority::from_str("p1").unwrap(), Priority::High);
      assert_eq!(Priority::from_str("medium").unwrap(), Priority::Medium);
      assert_eq!(Priority::from_str("p2").unwrap(), Priority::Medium);
      assert_eq!(Priority::from_str("low").unwrap(), Priority::Low);
      assert_eq!(Priority::from_str("p3").unwrap(), Priority::Low);
      assert_eq!(Priority::from_str("lowest").unwrap(), Priority::Lowest);
      assert_eq!(Priority::from_str("p4").unwrap(), Priority::Lowest);
    }

    #[test]
    fn it_errors_on_invalid_input() {
      assert!(Resolution::from_str("invalid").is_err());
    }

    #[test]
    fn it_parses_case_insensitively() {
      for str in ["P0", "CRITICAL", "Critical", "CrItIcAl"] {
        assert_eq!(Priority::from_str(str).unwrap(), Priority::Critical);
      }
    }
  }

  mod ordering {
    use super::*;

    #[test]
    fn it_orders() {
      assert!(Priority::Critical > Priority::High);
      assert!(Priority::High > Priority::Medium);
      assert!(Priority::Medium > Priority::Low);
      assert!(Priority::Low > Priority::Lowest);
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
        .execute("CREATE TABLE test (priority TEXT)", [])
        .unwrap();

      for priority in [
        Priority::Critical,
        Priority::High,
        Priority::Medium,
        Priority::Low,
        Priority::Lowest,
      ] {
        connection
          .execute("INSERT INTO test (priority) VALUES (?1)", [priority])
          .unwrap();
      }

      let mut statement = connection
        .prepare("SELECT * FROM test WHERE priority = ?1")
        .unwrap();
      let critical: Priority = statement
        .query_row([Priority::Critical], |row| row.get(0))
        .unwrap();
      let high: Priority = statement
        .query_row([Priority::High], |row| row.get(0))
        .unwrap();
      let medium: Priority = statement
        .query_row([Priority::Medium], |row| row.get(0))
        .unwrap();
      let low: Priority = statement
        .query_row([Priority::Low], |row| row.get(0))
        .unwrap();
      let lowest: Priority = statement
        .query_row([Priority::Lowest], |row| row.get(0))
        .unwrap();

      assert_eq!(critical, Priority::Critical);
      assert_eq!(high, Priority::High);
      assert_eq!(medium, Priority::Medium);
      assert_eq!(low, Priority::Low);
      assert_eq!(lowest, Priority::Lowest);
    }
  }
}
