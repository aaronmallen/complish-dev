use serde::{Deserialize, Serialize};

use crate::entities::Task;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TaskList {
  pub name: String,
  pub tasks: Vec<Task>,
}
