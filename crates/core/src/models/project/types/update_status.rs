use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use diesel::{deserialize::FromSqlRow, expression::AsExpression};
use eyre::{Error, Result, eyre};

use crate::macros;

#[derive(AsExpression, Clone, Debug, Default, FromSqlRow, PartialEq)]
#[diesel(sql_type = diesel::sql_types::Text)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum UpdateStatus {
  AtRisk,
  OffTrack,
  #[default]
  OnTrack,
}

macros::impl_string_sql_traits!(UpdateStatus);

impl Display for UpdateStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::AtRisk => write!(f, "at_risk"),
      Self::OffTrack => write!(f, "off_track"),
      Self::OnTrack => write!(f, "on_track"),
    }
  }
}

impl FromStr for UpdateStatus {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s.to_lowercase().as_str() {
      "at_risk" => Ok(Self::AtRisk),
      "off_track" => Ok(Self::OffTrack),
      "on_track" => Ok(Self::OnTrack),
      _ => Err(eyre!("Invalid update status: {}", s)),
    }
  }
}
