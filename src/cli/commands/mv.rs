use std::path::PathBuf;

use clap::Args;
use eyre::Result;

use crate::{config::Config, services::VaultService};

/// Move the complish vault
#[derive(Args, Debug)]
pub struct Mv {
  /// The path to move the vault to
  pub path: PathBuf,
}

impl Mv {
  pub fn run(&self) -> Result<()> {
    let mut config = Config::load();
    let old_path = config.vault.path.clone();
    config.vault.path.clone_from(&self.path);
    config.save()?;

    VaultService::mv(&old_path, &self.path)?;
    println!("Moved complish vault to {}", self.path.display());
    Ok(())
  }
}
