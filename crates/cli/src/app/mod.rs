mod commands;

use clap::Parser;
use color_eyre::Result;
use complish::env::Value as EnvValue;
use yansi::Paint;

use crate::ui::{alert, text};

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
  command: commands::Commands,
}

impl App {
  pub fn run() -> Result<()> {
    Self::bootstrap()?;
    let app = Self::parse();
    app.command.run()
  }

  fn bootstrap() -> Result<()> {
    Self::check_env();
    complish::store::connect()?;

    Ok(())
  }

  fn check_env() {
    if let EnvValue::Err(path) = complish::env::COMPLISH_DATA_DIR.value() {
      alert::warn(format!(
        "{} is not an absolute path: {:?}",
        text::info("COMPLISH_DATA_DIR").underline(),
        path
      ))
    }
  }
}
