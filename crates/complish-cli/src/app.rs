use clap::Parser;
use complish::vault::repo::Repo;
use eyre::Result;

use crate::commands::Commands;

/// The Complish CLI
#[derive(Debug, Parser)]
#[command(
  name = "complish",
  about = "A personal productivity tool for managing tasks",
  author = "Aaron Allen <hello@aaronmallen.me>",
  disable_version_flag = true
)]
pub struct App {
  #[command(subcommand)]
  command: Commands,
}

impl App {
  pub fn run(repo: &Repo) -> Result<()> {
    let app = Self::parse();
    app.command.run(repo)
  }
}
