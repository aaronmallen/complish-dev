use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types::Text};
use eyre::{Error, Result, eyre};

use crate::macros;

#[derive(AsExpression, Clone, Debug, Default, Eq, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum WorkflowStatus {
  Done,
  InProgress,
  Planned,
  #[default]
  Todo,
}

macros::impl_string_sql_traits!(WorkflowStatus);

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
