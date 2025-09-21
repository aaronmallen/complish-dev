mod commands;

use clap::Args;
use color_eyre::{Result, eyre::eyre};
use commands::Commands;
use complish::Task as TaskEntity;

use crate::ui::TaskDetail;

/// Create, list, and manage tasks
#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct Task {
  #[command(subcommand)]
  command: Option<Commands>,
  id: Option<i32>,
}

impl Task {
  pub fn run(self) -> Result<()> {
    if let Some(cmd) = self.command {
      return cmd.run();
    }

    if let Some(id) = self.id {
      let task =
        TaskEntity::find_by_sequence_id(id).map_err(|_| eyre!("Task #{} not found", id))?;
      TaskDetail::new(task).render();
      return Ok(());
    }

    Err(eyre!("No command or id provided"))
  }
}
