use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use chrono::Duration;
use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types::Text};
use eyre::{Error, Result, eyre};
use humantime::{format_duration, parse_duration};

use crate::macros;

#[derive(AsExpression, Clone, Debug, Eq, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum Estimation {
  Points(u8),
  Time(Duration),
}

macros::impl_string_sql_traits!(Estimation);

impl Display for Estimation {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Points(points) => write!(f, "{points}pts"),
      Self::Time(time) => match time.to_std() {
        Ok(std_duration) => write!(f, "{}", format_duration(std_duration)),
        Err(_) => write!(f, "Invalid duration"),
      },
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
      return Ok(Self::Points(points));
    }

    match parse_duration(s) {
      Ok(std_duration) => match Duration::from_std(std_duration) {
        Ok(chrono_duration) => Ok(Self::Time(chrono_duration)),
        Err(_) => Err(eyre!("Invalid duration: too large")),
      },
      Err(_) => Err(eyre!("Invalid estimation format: {}", s)),
    }
  }
}
