use std::{
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[serde(rename_all = "snake_case")]
pub enum Kind {
  Blocks,
  ChildOf,
  ClonedBy,
  ClonedFrom,
  DependsOn,
  ParentOf,
  RelatedTo,
}

impl_sql_string_traits!(Kind);

impl Display for Kind {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Blocks => write!(f, "blocks"),
      Self::ChildOf => write!(f, "child of"),
      Self::ClonedBy => write!(f, "cloned by"),
      Self::ClonedFrom => write!(f, "cloned from"),
      Self::DependsOn => write!(f, "depends on"),
      Self::ParentOf => write!(f, "parent of"),
      Self::RelatedTo => write!(f, "related to"),
    }
  }
}

impl FromStr for Kind {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s.to_lowercase().as_str() {
      "blocks" => Ok(Self::Blocks),
      "child of" | "child_of" => Ok(Self::ChildOf),
      "cloned by" | "cloned_by" => Ok(Self::ClonedBy),
      "cloned from" | "cloned_from" => Ok(Self::ClonedFrom),
      "depends on" | "depends_on" => Ok(Self::DependsOn),
      "parent of" | "parent_of" => Ok(Self::ParentOf),
      "related to" | "related_to" => Ok(Self::RelatedTo),
      _ => Err(eyre!("invalid relationship type")),
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
      assert_eq!(Kind::Blocks.to_string(), "blocks");
      assert_eq!(Kind::ChildOf.to_string(), "child of");
      assert_eq!(Kind::ClonedBy.to_string(), "cloned by");
      assert_eq!(Kind::ClonedFrom.to_string(), "cloned from");
      assert_eq!(Kind::DependsOn.to_string(), "depends on");
      assert_eq!(Kind::ParentOf.to_string(), "parent of");
      assert_eq!(Kind::RelatedTo.to_string(), "related to");
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
      let blocks: Kind = statement.query_row(["blocks"], |row| row.get(0)).unwrap();
      let child_of: Kind = statement.query_row(["child of"], |row| row.get(0)).unwrap();
      let cloned_by: Kind = statement
        .query_row(["cloned by"], |row| row.get(0))
        .unwrap();
      let cloned_from: Kind = statement
        .query_row(["cloned from"], |row| row.get(0))
        .unwrap();
      let depends_on: Kind = statement
        .query_row(["depends on"], |row| row.get(0))
        .unwrap();
      let parent_of: Kind = statement
        .query_row(["parent of"], |row| row.get(0))
        .unwrap();
      let related_to: Kind = statement
        .query_row(["related to"], |row| row.get(0))
        .unwrap();

      assert_eq!(blocks, Kind::Blocks);
      assert_eq!(child_of, Kind::ChildOf);
      assert_eq!(cloned_by, Kind::ClonedBy);
      assert_eq!(cloned_from, Kind::ClonedFrom);
      assert_eq!(depends_on, Kind::DependsOn);
      assert_eq!(parent_of, Kind::ParentOf);
      assert_eq!(related_to, Kind::RelatedTo);
    }
  }

  mod from_str {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses() {
      assert_eq!(Kind::from_str("blocks").unwrap(), Kind::Blocks);

      for str in &["child of", "child_of"] {
        assert_eq!(Kind::from_str(str).unwrap(), Kind::ChildOf);
      }

      for str in &["cloned by", "cloned_by"] {
        assert_eq!(Kind::from_str(str).unwrap(), Kind::ClonedBy);
      }

      for str in &["cloned from", "cloned_from"] {
        assert_eq!(Kind::from_str(str).unwrap(), Kind::ClonedFrom);
      }

      for str in &["depends on", "depends_on"] {
        assert_eq!(Kind::from_str(str).unwrap(), Kind::DependsOn);
      }

      for str in &["parent of", "parent_of"] {
        assert_eq!(Kind::from_str(str).unwrap(), Kind::ParentOf);
      }

      for str in &["related to", "related_to"] {
        assert_eq!(Kind::from_str(str).unwrap(), Kind::RelatedTo);
      }
    }

    #[test]
    fn it_errors_on_invalid_input() {
      assert!(Kind::from_str("invalid").is_err());
    }

    #[test]
    fn it_parses_case_insensitively() {
      for str in ["BLOCKS", "Blocks", "BlOcKs"] {
        assert_eq!(Kind::from_str(str).unwrap(), Kind::Blocks);
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
        .execute("CREATE TABLE test (relationship_type TEXT)", [])
        .unwrap();

      for relationship_type in [
        Kind::Blocks,
        Kind::ChildOf,
        Kind::ClonedBy,
        Kind::ClonedFrom,
        Kind::DependsOn,
        Kind::ParentOf,
        Kind::RelatedTo,
      ] {
        connection
          .execute(
            "INSERT INTO test (relationship_type) VALUES (?1)",
            [relationship_type],
          )
          .unwrap();
      }

      let mut statement = connection
        .prepare("SELECT * FROM test where relationship_type = ?1")
        .unwrap();

      let blocks: Kind = statement
        .query_row([Kind::Blocks], |row| row.get(0))
        .unwrap();
      let child_of: Kind = statement
        .query_row([Kind::ChildOf], |row| row.get(0))
        .unwrap();
      let cloned_by: Kind = statement
        .query_row([Kind::ClonedBy], |row| row.get(0))
        .unwrap();
      let cloned_from: Kind = statement
        .query_row([Kind::ClonedFrom], |row| row.get(0))
        .unwrap();
      let depends_on: Kind = statement
        .query_row([Kind::DependsOn], |row| row.get(0))
        .unwrap();
      let parent_of: Kind = statement
        .query_row([Kind::ParentOf], |row| row.get(0))
        .unwrap();
      let related_to: Kind = statement
        .query_row([Kind::RelatedTo], |row| row.get(0))
        .unwrap();

      assert_eq!(blocks, Kind::Blocks);
      assert_eq!(child_of, Kind::ChildOf);
      assert_eq!(cloned_by, Kind::ClonedBy);
      assert_eq!(cloned_from, Kind::ClonedFrom);
      assert_eq!(depends_on, Kind::DependsOn);
      assert_eq!(parent_of, Kind::ParentOf);
      assert_eq!(related_to, Kind::RelatedTo);
    }
  }
}
