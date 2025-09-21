mod create;
mod list;

use clap::Subcommand;
use color_eyre::Result;

#[derive(Debug, Subcommand)]
pub enum Commands {
  #[command(alias = "add")]
  Create(create::Create),
  List(list::List),
}

impl Commands {
  pub fn run(self) -> Result<()> {
    match self {
      Self::Create(cmd) => cmd.run(),
      Self::List(cmd) => cmd.run(),
    }
  }
}
