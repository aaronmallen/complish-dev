use std::fmt::{Display, Formatter};

use crate::{entities::Task, ui::CompactTaskDisplay};

pub struct TaskListDisplay {
  indent: usize,
  list_name: String,
  tasks: Vec<Task>,
}

impl Display for TaskListDisplay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}\n{}", self.title(), self.task_list())
  }
}

impl TaskListDisplay {
  pub fn new(list_name: &str, tasks: Vec<Task>) -> Self {
    Self {
      indent: 0,
      list_name: list_name.to_string(),
      tasks,
    }
  }

  pub fn with_indent(mut self, indent: usize) -> Self {
    self.indent = indent;
    self
  }

  fn task_list(&self) -> String {
    let max_width = self.tasks.iter().map(|t| t.display_id.len()).max().unwrap_or(0);

    self
      .tasks
      .iter()
      .map(|task| {
        CompactTaskDisplay::new(task.clone()).with_max_id_width(max_width).with_indent(self.indent + 2) // Tasks get extra indentation
      })
      .map(|display| display.to_string())
      .collect::<Vec<_>>()
      .join("\n")
  }

  fn title(&self) -> String {
    format!(
      "{}{} ({} {})",
      " ".repeat(self.indent),
      self.list_name,
      self.tasks.len(),
      if self.tasks.len() == 1 {
        "task"
      } else {
        "tasks"
      }
    )
  }
}
