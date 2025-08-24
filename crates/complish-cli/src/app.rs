use clap::Parser;
use eyre::Result;

/// The Complish CLI
#[derive(Debug, Parser)]
#[command(
  name = "complish",
  about = "A personal productivity tool for managing tasks",
  author = "Aaron Allen <hello@aaronmallen.me>",
  disable_version_flag = true
)]
pub struct App;

impl App {
  pub fn run() -> Result<()> {
    let app = Self::parse();
    todo!()
  }
}
