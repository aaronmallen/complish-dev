use std::collections::HashMap;

use clap::Args;
use eyre::Result;

use crate::{
  entities::Task,
  repos::{ProjectRepo, TaskListRepo},
  ui::{MultiTaskListDisplay, TaskListDisplay},
};

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
      self.print_scoped_project()?;
    } else if self.project.is_some() {
      self.print_project()?;
    } else {
      self.print_list()?;
    }

    Ok(())
  }

  fn print_list(&self) -> Result<()> {
    let list_name = self.list.as_ref().unwrap_or(&"today".to_string()).clone();
    let repo = TaskListRepo::new();
    let list = repo.by_name(&list_name)?;

    println!("{}", TaskListDisplay::new(&list.name, list.tasks));
    Ok(())
  }

  fn print_project(&self) -> Result<()> {
    let repo = ProjectRepo::new();
    let project = repo.by_id(self.project.as_ref().unwrap())?;

    let mut lists = HashMap::<String, Vec<Task>>::new();
    for task in &project.tasks {
      lists.entry(task.list_name.clone()).or_default().push(task.clone());
    }

    let mut display = MultiTaskListDisplay::new(&project.name);
    for (list_name, tasks) in lists {
      display = display.add_task_list(&list_name, tasks);
    }

    println!("{display}");
    Ok(())
  }

  fn print_scoped_project(&self) -> Result<()> {
    let repo = ProjectRepo::new();
    let project = repo.by_id(self.project.as_ref().unwrap())?;
    let list_name = self.list.as_ref().unwrap();
    let filtered_tasks: Vec<_> = project.tasks.into_iter().filter(|task| &task.list_name == list_name).collect();
    let display = MultiTaskListDisplay::new(&project.name).add_task_list(list_name, filtered_tasks);

    println!("{display}");
    Ok(())
  }
}
