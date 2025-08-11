use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
  Delegated,
  Done,
  #[serde(rename = "in_progress")]
  InProgress,
  #[default]
  Todo,
}
