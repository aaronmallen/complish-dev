use std::{
  cmp::Ordering,
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types::Text};
use eyre::{Error, Result, eyre};

use crate::macros;

#[derive(AsExpression, Clone, Debug, Default, Eq, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum Priority {
  #[cfg_attr(feature = "cli", clap(name = "p0", alias = "critical"))]
  Critical = 0,
  #[cfg_attr(feature = "cli", clap(name = "p1", alias = "high"))]
  High = 1,
  #[cfg_attr(feature = "cli", clap(name = "p2", alias = "medium"))]
  #[default]
  Medium = 2,
  #[cfg_attr(feature = "cli", clap(name = "p3", alias = "low"))]
  Low = 3,
  #[cfg_attr(feature = "cli", clap(name = "p4", alias = "lowest"))]
  Lowest = 4,
}

impl Priority {
  pub fn as_named_str(&self) -> &str {
    match self {
      Self::Critical => "critical",
      Self::High => "high",
      Self::Medium => "medium",
      Self::Low => "low",
      Self::Lowest => "lowest",
    }
  }
}

macros::impl_string_sql_traits!(Priority);

impl Display for Priority {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Critical => write!(f, "p0"),
      Self::High => write!(f, "p1"),
      Self::Medium => write!(f, "p2"),
      Self::Low => write!(f, "p3"),
      Self::Lowest => write!(f, "p4"),
    }
  }
}

impl FromStr for Priority {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s.to_lowercase().as_str() {
      "critical" | "p0" => Ok(Self::Critical),
      "high" | "p1" => Ok(Self::High),
      "medium" | "p2" => Ok(Self::Medium),
      "low" | "p3" => Ok(Self::Low),
      "lowest" | "p4" => Ok(Self::Lowest),
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
