mod commands;

use clap::Args;
use color_eyre::{Result, eyre::eyre};
use commands::Commands;

/// Create, list, and manage tasks
#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct Task {
  #[command(subcommand)]
  command: Option<Commands>,
  id: Option<u32>,
}

impl Task {
  pub fn run(self) -> Result<()> {
    if let Some(cmd) = self.command {
      return cmd.run();
    }

    if let Some(id) = self.id {
      todo!()
    }

    Err(eyre!("No command or id provided"))
  }
}
