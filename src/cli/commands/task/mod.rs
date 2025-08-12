mod create;

use clap::{Args, Subcommand};
use eyre::Result;

/// Manage complish tasks
#[derive(Args, Debug)]
pub struct Task {
  #[command(subcommand)]
  pub command: Option<TaskCommand>,
}

#[derive(Debug, Subcommand)]
pub enum TaskCommand {
  Add(create::Create),
  Create(create::Create),
}

impl Task {
  pub fn run(self) -> Result<()> {
    match self.command {
      Some(TaskCommand::Add(cmd) | TaskCommand::Create(cmd)) => cmd.run(),
      None => {
        println!("coming soon");
        Ok(())
      }
    }
  }
}
