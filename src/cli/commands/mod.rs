#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_self)]

use clap::Subcommand;
use eyre::Result;

mod version;

/// Complish CLI commands
#[derive(Debug, Subcommand)]
pub enum Commands {
  #[command(long_flag = "version", short_flag = 'v')]
  Version(version::Version),
}

impl Commands {
  pub fn run(self) -> Result<()> {
    match self {
      Commands::Version(cmd) => cmd.run(),
    }
  }
}
