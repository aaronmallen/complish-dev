use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
  time::Duration,
};

use eyre::{Error, Result, eyre};
use humantime::{format_duration, parse_duration};
use rusqlite::{
  Result as SqliteResult,
  types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
};
use serde::{Deserialize, Serialize};

use crate::macros::impl_sql_string_traits;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Estimation {
  Points(u8),
  Time(Duration),
}

impl_sql_string_traits!(Estimation);

impl Display for Estimation {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Estimation::Points(points) => write!(f, "{points} points"),
      Estimation::Time(time) => write!(f, "{}", format_duration(*time)),
    }
  }
}

impl FromStr for Estimation {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let trimmed = s.trim().to_lowercase();

    if let Some(points_str) = trimmed
      .strip_suffix("points")
      .or_else(|| trimmed.strip_suffix("pts"))
      .or_else(|| trimmed.strip_suffix("pt"))
      .or_else(|| trimmed.strip_suffix('p'))
      && let Ok(points) = points_str.trim().parse::<u8>()
    {
      return Ok(Estimation::Points(points));
    }

    match parse_duration(s) {
      Ok(duration) => Ok(Estimation::Time(duration)),
      Err(_) => Err(eyre!("Invalid estimation format: {}", s)),
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
      assert_eq!(Estimation::Points(1).to_string(), "1 points");
      assert_eq!(Estimation::Time(Duration::from_secs(1)).to_string(), "1s");
      assert_eq!(
        Estimation::Time(Duration::from_secs(172_800)).to_string(),
        "2days"
      );
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
      let points: Estimation = statement.query_row(["2 points"], |row| row.get(0)).unwrap();
      let duration: Estimation = statement.query_row(["60s"], |row| row.get(0)).unwrap();

      assert_eq!(points, Estimation::Points(2));
      assert_eq!(duration, Estimation::Time(Duration::from_secs(60)));
    }
  }

  mod from_str {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses() {
      assert_eq!(
        Estimation::from_str("2 points").unwrap(),
        Estimation::Points(2)
      );
      assert_eq!(
        Estimation::from_str("60s").unwrap(),
        Estimation::Time(Duration::from_secs(60))
      );
      assert_eq!(
        Estimation::from_str("2days").unwrap(),
        Estimation::Time(Duration::from_secs(172_800))
      );
    }

    #[test]
    fn it_errors_on_invalid_input() {
      assert!(Estimation::from_str("invalid").is_err());
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
        .execute("CREATE TABLE test (estimation TEXT)", [])
        .unwrap();

      for estimation in [
        Estimation::Points(2),
        Estimation::Time(Duration::from_secs(60)),
      ] {
        connection
          .execute("INSERT INTO test (estimation) VALUES (?1)", [estimation])
          .unwrap();
      }

      let mut statement = connection
        .prepare("SELECT * FROM test where estimation = ?1")
        .unwrap();

      let points: Estimation = statement
        .query_row([Estimation::Points(2)], |row| row.get(0))
        .unwrap();
      let duration: Estimation = statement
        .query_row([Estimation::Time(Duration::from_secs(60))], |row| {
          row.get(0)
        })
        .unwrap();

      assert_eq!(points, Estimation::Points(2));
      assert_eq!(duration, Estimation::Time(Duration::from_secs(60)));
    }
  }
}
