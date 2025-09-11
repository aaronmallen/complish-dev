mod version;

use clap::Subcommand;
use color_eyre::Result;

#[derive(Debug, Subcommand)]
pub enum Commands {
  #[command(long_flag = "version", short_flag = 'v')]
  Version(version::Version),
}

impl Commands {
  pub fn run(self) -> Result<()> {
    match self {
      Self::Version(cmd) => cmd.run(),
    }
  }
}
