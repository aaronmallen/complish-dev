use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use diesel::{deserialize::FromSqlRow, expression::AsExpression};
use eyre::{Error, Result, eyre};

use crate::macros;

#[derive(AsExpression, Clone, Debug, Eq, FromSqlRow, Hash, PartialEq)]
#[diesel(sql_type = diesel::sql_types::Text)]
#[derive(Default)]
pub enum RelationshipKind {
  Blocks,
  Depends,
  #[default]
  Relates,
}

macros::impl_string_sql_traits!(RelationshipKind);

impl Display for RelationshipKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Blocks => write!(f, "blocks"),
      Self::Depends => write!(f, "depends"),
      Self::Relates => write!(f, "relates"),
    }
  }
}

impl FromStr for RelationshipKind {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s.to_lowercase().as_str() {
      "blocks" => Ok(Self::Blocks),
      "depends" => Ok(Self::Depends),
      "relates" => Ok(Self::Relates),
      _ => Err(eyre!("Invalid relationship kind: {}", s)),
    }
  }
}
