use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use eyre::{Error, Result, eyre};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[serde(rename_all = "snake_case")]
pub enum UpdateStatus {
  AtRisk,
  OffTrack,
  OnTrack,
}

impl Display for UpdateStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::AtRisk => write!(f, "at risk"),
      Self::OffTrack => write!(f, "off track"),
      Self::OnTrack => write!(f, "on track"),
    }
  }
}

impl FromStr for UpdateStatus {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "at risk" | "at_risk" => Ok(Self::AtRisk),
      "off track" | "off_track" => Ok(Self::OffTrack),
      "on track" | "on_track" => Ok(Self::OnTrack),
      _ => Err(eyre!("invalid update status: {}", s)),
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
      assert_eq!(UpdateStatus::AtRisk.to_string(), "at risk");
      assert_eq!(UpdateStatus::OffTrack.to_string(), "off track");
      assert_eq!(UpdateStatus::OnTrack.to_string(), "on track");
    }
  }

  mod from_str {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses() {
      for str in ["at risk", "at_risk"] {
        assert_eq!(UpdateStatus::from_str(str).unwrap(), UpdateStatus::AtRisk);
      }

      for str in ["off track", "off_track"] {
        assert_eq!(UpdateStatus::from_str(str).unwrap(), UpdateStatus::OffTrack);
      }

      for str in ["on track", "on_track"] {
        assert_eq!(UpdateStatus::from_str(str).unwrap(), UpdateStatus::OnTrack);
      }
    }

    #[test]
    fn it_errors_on_invalid_input() {
      assert!(UpdateStatus::from_str("invalid").is_err());
    }

    #[test]
    fn it_parses_case_insensitively() {
      for str in [
        "AT_RISK", "AT RISK", "At_Risk", "At Risk", "At_RiSk", "At RiSk",
      ] {
        assert_eq!(UpdateStatus::from_str(str).unwrap(), UpdateStatus::AtRisk);
      }
    }
  }
}
