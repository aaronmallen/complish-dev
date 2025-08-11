mod get;
mod set;
mod unset;

use clap::Subcommand;
use eyre::Result;

/// Manage the complish configuration
#[derive(Debug, Subcommand)]
pub enum Config {
  Get(get::Get),
  Set(set::Set),
  Unset(unset::Unset),
}

impl Config {
  pub fn run(self) -> Result<()> {
    match self {
      Config::Get(cmd) => cmd.run(),
      Config::Set(cmd) => cmd.run(),
      Config::Unset(cmd) => cmd.run(),
    }
  }
}
