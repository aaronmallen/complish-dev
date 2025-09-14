use chrono::NaiveDateTime;
use clap::Args;
use color_eyre::{Result, eyre::eyre};
use complish::{Task, TaskPriority};

use crate::ui::text;

/// Create a new task
#[derive(Args, Debug)]
pub struct Create {
  /// The title of the task
  title: String,
  /// The description of the task
  #[arg(long)]
  description: Option<String>,
  /// The due date of the task
  #[arg(long, allow_hyphen_values = true, value_parser = parse_naive_datetime)]
  due: Option<NaiveDateTime>,
  /// The priority of the task
  #[arg(long, short = 'p')]
  priority: Option<TaskPriority>,
}

fn parse_naive_datetime(s: &str) -> Result<NaiveDateTime> {
  let formats = [
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%d %H:%M",
    "%Y-%m-%dT%H:%M",
    "%Y/%m/%d %H:%M:%S",
    "%Y/%m/%d %H:%M",
    "%d-%m-%Y %H:%M:%S",
    "%d-%m-%Y %H:%M",
    "%d/%m/%Y %H:%M:%S",
    "%d/%m/%Y %H:%M",
    "%Y-%m-%d",
    "%Y/%m/%d",
    "%d-%m-%Y",
    "%d/%m/%Y",
  ];

  for format in formats {
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, format) {
      return Ok(dt);
    }
    if !format.contains("%H")
      && let Ok(date) = chrono::NaiveDate::parse_from_str(s, format)
    {
      return Ok(date.and_hms_opt(0, 0, 0).unwrap());
    }
  }

  Err(eyre!(
    "Invalid datetime format. Examples: '2024-12-25 14:30:00', '2024-12-25T14:30', '25/12/2024 14:30', '2024-12-25'"
  ))
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
    let saved_task = Task::find(task.id())?;

    println!(
      "{} created task #{}",
      text::success("✔"),
      saved_task
        .sequence_id()
        .ok_or_else(|| eyre!("Something went wrong"))?
    );
    Ok(())
  }
}
