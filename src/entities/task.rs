use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::{Priority, TaskStatus};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Task {
  pub completed: Option<DateTime<Utc>>,
  pub created: DateTime<Utc>,
  pub description: Option<String>,
  pub display_id: String,
  pub due_date: Option<DateTime<Utc>>,
  pub id: u32,
  pub priority: Priority,
  pub project_id: Option<String>,
  pub status: TaskStatus,
  pub subject: String,
  pub list_name: String,
  pub updated: DateTime<Utc>,
}

impl Default for Task {
  fn default() -> Self {
    let now = Utc::now();

    Self {
      completed: None,
      created: now,
      description: None,
      display_id: String::new(),
      due_date: None,
      id: 0,
      priority: Priority::default(),
      project_id: None,
      status: TaskStatus::default(),
      subject: String::new(),
      list_name: String::new(),
      updated: now,
    }
  }
}
