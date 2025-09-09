use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types::Text};
use eyre::{Error, Result, eyre};

use crate::macros;

#[derive(AsExpression, Clone, Debug, Eq, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum Resolution {
  Canceled,
  Completed,
}

macros::impl_string_sql_traits!(Resolution);

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
