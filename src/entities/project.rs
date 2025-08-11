use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::Task;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Project {
  pub completed: Option<DateTime<Utc>>,
  pub created: DateTime<Utc>,
  pub description: Option<String>,
  pub id: String,
  pub name: String,
  pub tasks: Vec<Task>,
  pub updated: DateTime<Utc>,
}

impl Default for Project {
  fn default() -> Self {
    let now = Utc::now();

    Self {
      completed: None,
      created: now,
      description: None,
      id: String::new(),
      name: String::new(),
      tasks: Vec::new(),
      updated: now,
    }
  }
}
