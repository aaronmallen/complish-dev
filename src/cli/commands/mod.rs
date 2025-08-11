#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_self)]

use clap::Subcommand;
use eyre::Result;

mod config;
mod init;
mod version;

/// Complish CLI commands
#[derive(Debug, Subcommand)]
pub enum Commands {
  #[command(subcommand)]
  Config(config::Config),
  Init(init::Init),
  #[command(long_flag = "version", short_flag = 'v')]
  Version(version::Version),
}

impl Commands {
  pub fn run(self) -> Result<()> {
    match self {
      Commands::Config(cmd) => cmd.run(),
      Commands::Init(cmd) => cmd.run(),
      Commands::Version(cmd) => cmd.run(),
    }
  }
}
