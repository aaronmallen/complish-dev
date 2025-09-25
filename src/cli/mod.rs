mod commands;

use clap::Parser;
use color_eyre::Result;

#[derive(Debug, Parser)]
#[command(
  name = "complish",
  about = "A command line tool for managing your accomplishments.",
  author = "Aaron Allen <hello@aaronmallen.me>",
  version = env!("CARGO_PKG_VERSION"),
)]
pub struct Cli {
  #[command(subcommand)]
  command: commands::Commands,
}

impl Cli {
  pub fn run() -> Result<()> {
    let cli = Self::parse();
    cli.command.run()
  }
}
