use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Priority {
  #[serde(rename = "p0")]
  Highest = 0,
  #[serde(rename = "p1")]
  High = 1,
  #[default]
  #[serde(rename = "p2")]
  Medium = 2,
  #[serde(rename = "p3")]
  Low = 3,
  #[serde(rename = "p4")]
  Lowest = 4,
}

impl Ord for Priority {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    (other.clone() as u8).cmp(&(self.clone() as u8))
  }
}

impl PartialOrd for Priority {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
