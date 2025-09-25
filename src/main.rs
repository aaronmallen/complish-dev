mod cli;
mod models;
mod parsers;
mod store;
mod ui;

use color_eyre::Result;

fn main() -> Result<()> {
  store::connect()?;

  if let Err(e) = cli::Cli::run() {
    eprintln!("Error: {}", e);
    std::process::exit(1)
  }

  Ok(())
}
