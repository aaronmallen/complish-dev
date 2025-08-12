use std::fmt::{Display, Formatter};

use crate::{entities::Task, ui::TaskListDisplay};

pub struct MultiTaskListDisplay {
  list_name: String,
  task_lists: Vec<TaskListDisplay>,
  total_tasks: usize,
}

impl Display for MultiTaskListDisplay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}\n{}", self.title(), self.lists_list())
  }
}

impl MultiTaskListDisplay {
  pub fn new(list_name: &str) -> Self {
    Self {
      list_name: list_name.to_string(),
      task_lists: Vec::new(),
      total_tasks: 0,
    }
  }

  pub fn add_task_list(mut self, list_name: &str, tasks: Vec<Task>) -> Self {
    self.total_tasks += tasks.len();
    self.task_lists.push(TaskListDisplay::new(list_name, tasks).with_indent(2));
    self
  }

  fn title(&self) -> String {
    format!(
      "{} ({} {})",
      self.list_name,
      self.total_tasks,
      if self.total_tasks == 1 {
        "task"
      } else {
        "tasks"
      }
    )
  }

  fn lists_list(&self) -> String {
    self.task_lists.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join("\n\n")
  }
}
