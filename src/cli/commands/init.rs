use std::path::PathBuf;

use clap::Args;
use eyre::Result;

use crate::{config::Config, services::VaultService};

/// Initialize the complish vault
#[derive(Args, Debug)]
pub struct Init {
  /// The path to initialize the vault at
  pub path: PathBuf,
}

impl Init {
  pub fn run(&self) -> Result<()> {
    VaultService::init(&self.path)?;

    let mut config = Config::load();
    config.vault.path.clone_from(&self.path);
    config.save()?;

    println!("Created complish vault at: {}", self.path.display());
    Ok(())
  }
}
