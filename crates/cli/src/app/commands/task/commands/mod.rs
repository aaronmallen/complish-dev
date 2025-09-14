mod create;

use clap::Subcommand;
use color_eyre::Result;

#[derive(Debug, Subcommand)]
pub enum Commands {
  #[command(alias = "add")]
  Create(create::Create),
}

impl Commands {
  pub fn run(self) -> Result<()> {
    match self {
      Self::Create(cmd) => cmd.run(),
    }
  }
}
