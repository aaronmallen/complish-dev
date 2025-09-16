use chrono::NaiveDateTime;
use clap::Args;
use color_eyre::{Result, eyre::eyre};
use complish::{Tag, Task, TaskPriority};
use yansi::Paint;

use crate::ui::{alert, color, text};

/// Create a new task
#[derive(Args, Debug)]
pub struct Create {
  /// The title of the task
  title: String,
  /// The description of the task
  #[arg(long)]
  description: Option<String>,
  /// The due date of the task
  #[arg(long, allow_hyphen_values = true, value_parser = crate::value_parser::naive_datetime)]
  due: Option<NaiveDateTime>,
  /// The priority of the task
  #[arg(long, short = 'p')]
  priority: Option<TaskPriority>,
  /// Tag the task
  #[arg(long)]
  tag: Vec<String>,
}

impl Create {
  pub fn run(self) -> Result<()> {
    let mut task = Task::new(self.title);

    if let Some(description) = self.description {
      task = task.with_description(description)
    }

    if let Some(due_at) = self.due {
      task = task.with_due_date(due_at);
    }

    if let Some(priority) = self.priority {
      task = task.with_priority(priority);
    }

    task.save()?;
    let mut saved_task = Task::find(task.id())?;

    for tag in self.tag {
      let db_tag = Tag::find_or_create(tag)?;
      saved_task.add_tag(db_tag.label())?;
    }

    let sequence_id_string = format!(
      " #{} ",
      saved_task
        .sequence_id()
        .ok_or_else(|| eyre!("Something went wrong"))?
    )
      .bg(color::OFF_WHITE)
      .to_string();

    alert::success(format!(
      "{} created task {}",
      text::success("✔"),
      sequence_id_string
    ));
    Ok(())
  }
}
