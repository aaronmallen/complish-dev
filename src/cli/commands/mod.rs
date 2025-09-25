mod journal;
mod report;

use clap::Subcommand;
use color_eyre::Result;

#[derive(Debug, Subcommand)]
pub enum Commands {
  Journal(journal::Journal),
  Report(report::Report),
}

impl Commands {
  pub fn run(&self) -> Result<()> {
    match self {
      Self::Journal(cmd) => cmd.run(),
      Self::Report(cmd) => cmd.run(),
    }
  }
}
