mod create;
mod list;

use clap::{Args, Subcommand};
use eyre::Result;

/// Manage complish tasks
#[derive(Args, Debug)]
pub struct Task {
  #[command(subcommand)]
  pub command: Option<TaskCommand>,
  /// The task list to list tasks for (default: "today")
  #[arg(long)]
  pub list: Option<String>,
  /// The project id to list tasks for
  #[arg(long)]
  pub project: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum TaskCommand {
  Add(create::Create),
  Create(create::Create),
  List(list::List),
}

impl Task {
  pub fn run(self) -> Result<()> {
    match self.command {
      Some(TaskCommand::Add(cmd) | TaskCommand::Create(cmd)) => cmd.run(),
      Some(TaskCommand::List(cmd)) => cmd.run(),
      None => {
        let cmd = list::List {
          list: self.list,
          project: self.project,
        };
        cmd.run()
      }
    }
  }
}
