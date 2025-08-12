use chrono::{DateTime, Utc};
use clap::{Args, ValueEnum};
use eyre::Result;

use crate::{
  entities::{Priority, Task},
  repos::TaskRepo,
};

#[derive(Clone, Debug, ValueEnum)]
pub enum TaskPriority {
  P0,
  P1,
  P2,
  P3,
  P4,
}

/// Create a new task
#[derive(Args, Debug)]
pub struct Create {
  /// The task description
  #[arg(long, alias = "desc")]
  pub description: Option<String>,
  /// Set the due date of the task
  #[arg(long, value_parser = parse_user_datetime)]
  pub due: Option<DateTime<Utc>>,
  /// The task to add
  pub subject: String,
  /// The list to add the task to
  #[arg(long, default_value = "someday")]
  pub list: Option<String>,
  /// Set the priority of the task (p0=lowest, p4=highest)
  #[arg(long, value_enum)]
  pub priority: Option<TaskPriority>,
  /// The project to add the task to
  #[arg(long)]
  pub project: Option<String>,
}

fn parse_user_datetime(s: &str) -> Result<DateTime<Utc>, String> {
  let formats = ["%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M", "%Y-%m-%d", "%m/%d/%Y", "%m-%d-%Y %H:%M"];

  for format in formats {
    if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(s, format) {
      return Ok(naive_dt.and_utc());
    }
    if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(s, format) {
      return Ok(naive_date.and_hms_opt(0, 0, 0).unwrap().and_utc());
    }
  }

  Err(format!("Invalid date format: {s}"))
}

impl Create {
  pub fn run(&self) -> Result<()> {
    let repo = TaskRepo::new();

    let mut update_task = false;
    let mut task = self.create_task(&repo)?;

    if self.priority.is_some() {
      let task_priority = self.get_priority();
      update_task = true;
      task.priority = task_priority;
    }

    if self.due.is_some() {
      update_task = true;
      task.due_date = self.due;
    }

    if update_task {
      repo.update(task)?;
    }

    Ok(())
  }

  fn create_task(&self, repo: &TaskRepo) -> Result<Task> {
    repo.create(
      self.subject.as_str(),
      self.list.as_ref().unwrap().as_str(),
      self.description.clone(),
      self.project.as_deref(),
    )
  }

  fn get_priority(&self) -> Priority {
    match self.priority.clone().unwrap() {
      TaskPriority::P0 => Priority::Highest,
      TaskPriority::P1 => Priority::High,
      TaskPriority::P2 => Priority::Medium,
      TaskPriority::P3 => Priority::Low,
      TaskPriority::P4 => Priority::Lowest,
    }
  }
}
