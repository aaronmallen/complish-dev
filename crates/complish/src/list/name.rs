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
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[serde(rename_all = "snake_case")]
pub enum Name {
  #[cfg_attr(feature = "cli", clap(name = "next"))]
  Next,
  #[cfg_attr(feature = "cli", clap(name = "someday"))]
  #[default]
  Someday,
  #[cfg_attr(feature = "cli", clap(name = "today"))]
  Today,
}

impl Name {
  pub fn id(self) -> u8 {
    match self {
      Self::Next => 2,
      Self::Someday => 3,
      Self::Today => 1,
    }
  }
}

impl Display for Name {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Next => write!(f, "Next"),
      Self::Someday => write!(f, "Someday"),
      Self::Today => write!(f, "Today"),
    }
  }
}

impl FromSql for Name {
  fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
    let s = value.as_str()?;
    s.parse().map_err(|_| FromSqlError::InvalidType)
  }
}

impl FromStr for Name {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s.to_lowercase().as_str() {
      "next" => Ok(Self::Next),
      "someday" => Ok(Self::Someday),
      "today" => Ok(Self::Today),
      _ => Err(eyre!("Invalid name: {}", s)),
    }
  }
}

impl Ord for Name {
  fn cmp(&self, other: &Self) -> Ordering {
    self.id().cmp(&other.id())
  }
}

impl PartialOrd for Name {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl ToSql for Name {
  fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
    Ok(ToSqlOutput::from(self.to_string()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(feature = "cli")]
  mod clap_integration {
    use super::*;

    #[test]
    fn it_works_with_clap_value_enum() {
      use clap::ValueEnum;

      let variants = Name::value_variants();
      assert_eq!(variants.len(), 3);
      assert!(variants.contains(&Name::Today));
      assert!(variants.contains(&Name::Next));
      assert!(variants.contains(&Name::Someday));
    }
  }

  mod default {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_defaults_to_someday() {
      assert_eq!(Name::default(), Name::Someday);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_displays_correctly() {
      assert_eq!(Name::Today.to_string(), "Today");
      assert_eq!(Name::Next.to_string(), "Next");
      assert_eq!(Name::Someday.to_string(), "Someday");
    }
  }

  mod from_str {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses_correctly() {
      assert_eq!(Name::from_str("today").unwrap(), Name::Today);
      assert_eq!(Name::from_str("next").unwrap(), Name::Next);
      assert_eq!(Name::from_str("someday").unwrap(), Name::Someday);
    }

    #[test]
    fn it_errors_on_invalid_input() {
      assert!(Name::from_str("invalid").is_err());
      assert!(Name::from_str("").is_err());
      assert!(Name::from_str("tomorrow").is_err());
    }

    #[test]
    fn it_parses_case_insensitively() {
      // Today variations
      for str in &["TODAY", "Today", "ToDay", "tOdAy"] {
        assert_eq!(Name::from_str(str).unwrap(), Name::Today);
      }

      // Next variations
      for str in &["NEXT", "Next", "NeXt", "nExT"] {
        assert_eq!(Name::from_str(str).unwrap(), Name::Next);
      }

      // Someday variations
      for str in &["SOMEDAY", "Someday", "SomeDay", "sOmEdAy"] {
        assert_eq!(Name::from_str(str).unwrap(), Name::Someday);
      }
    }
  }

  mod id {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_correct_ids() {
      assert_eq!(Name::Today.id(), 1);
      assert_eq!(Name::Next.id(), 2);
      assert_eq!(Name::Someday.id(), 3);
    }

    #[test]
    fn ids_are_unique() {
      let ids = vec![Name::Today.id(), Name::Next.id(), Name::Someday.id()];
      let mut unique_ids = ids.clone();
      unique_ids.sort_unstable();
      unique_ids.dedup();
      assert_eq!(ids.len(), unique_ids.len());
    }
  }

  mod ordering {
    use super::*;

    #[test]
    fn it_orders_correctly() {
      assert!(Name::Today < Name::Next);
      assert!(Name::Next < Name::Someday);
      assert!(Name::Today < Name::Someday);
    }

    #[test]
    fn it_orders_by_id() {
      let mut names = vec![Name::Someday, Name::Today, Name::Next];
      names.sort();
      assert_eq!(names, vec![Name::Today, Name::Next, Name::Someday]);
    }
  }
}
