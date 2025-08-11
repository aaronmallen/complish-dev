use clap::Parser;
use eyre::Result;

mod commands;

/// The Complish CLI
#[derive(Debug, Parser)]
#[command(
  name = "complish",
  about = "A personal productivity tool for managing tasks",
  author = "Aaron Allen <hello@aaronmallen.me>",
  disable_version_flag = true
)]
pub struct CLI {
  #[command(subcommand)]
  commands: commands::Commands,
}

pub fn run() -> Result<()> {
  let cli = CLI::parse();
  cli.run()
}

impl CLI {
  pub fn run(self) -> Result<()> {
    self.commands.run()
  }
}
