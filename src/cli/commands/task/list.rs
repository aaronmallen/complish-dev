use clap::Args;
use eyre::Result;

use crate::repos::{ProjectRepo, TaskListRepo};

/// List all tasks in a given task list or project
#[derive(Args, Debug)]
pub struct List {
  /// The task list to list tasks for (default: "today")
  #[arg(long)]
  pub list: Option<String>,
  /// The project id to list tasks for
  #[arg(long)]
  pub project: Option<String>,
}

impl List {
  pub fn run(&self) -> Result<()> {
    if self.list.is_some() && self.project.is_some() {
      let repo = ProjectRepo::new();
      let project = repo.by_name(self.project.as_ref().unwrap())?;
      let list_name = self.list.as_ref().unwrap();

      let filtered_tasks: Vec<_> = project.tasks.into_iter().filter(|task| &task.list_name == list_name).collect();

      println!("{filtered_tasks:?}");
    } else if self.project.is_some() {
      let repo = ProjectRepo::new();
      let project = repo.by_name(self.project.as_ref().unwrap())?;
      println!("{:?}", project.tasks);
    } else {
      let list_name = self.list.as_ref().unwrap_or(&"today".to_string()).clone();
      let repo = TaskListRepo::new();
      let list = repo.by_name(&list_name)?;
      println!("{:?}", list.tasks);
    }

    Ok(())
  }
}
