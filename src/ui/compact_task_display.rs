use std::fmt::{Display, Formatter};

use crate::entities::{Task, TaskStatus};

pub struct CompactTaskDisplay {
  id_padding: usize,
  indent: usize,
  task: Task,
}

impl Display for CompactTaskDisplay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{}{}{} {}",
      " ".repeat(self.indent),
      self.icon(),
      " ".repeat(self.id_padding),
      self.id(),
      self.subject()
    )
  }
}

impl CompactTaskDisplay {
  pub fn new(task: Task) -> Self {
    Self {
      id_padding: 1,
      indent: 0,
      task,
    }
  }

  pub fn with_indent(mut self, indent: usize) -> Self {
    self.indent = indent;
    self
  }

  pub fn with_max_id_width(mut self, width: usize) -> Self {
    self.id_padding = width - self.task.display_id.len() + 1;
    self
  }

  fn icon(&self) -> &str {
    match self.task.status {
      TaskStatus::Delegated => "⦿",
      TaskStatus::Done => "●",
      TaskStatus::InProgress => "◐",
      TaskStatus::Todo => "○",
    }
  }

  fn id(&self) -> &str {
    &self.task.display_id
  }

  fn subject(&self) -> &str {
    &self.task.subject
  }
}
