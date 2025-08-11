use std::{collections::HashMap, path::PathBuf};

use eyre::Result;
use serde::{Deserialize, Serialize};
use toml::map::Map;

use crate::services::TomlLoaderService;

mod filepath;
mod vault;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
  pub vault: vault::Vault,

  #[serde(flatten)]
  pub extensions: HashMap<String, toml::Value>,
}

impl Config {
  pub fn filepath() -> PathBuf {
    filepath::FilePath::resolve()
  }

  pub fn load() -> Self {
    TomlLoaderService::load_or_default(&Self::filepath())
  }

  pub fn save(&self) -> Result<()> {
    let default_config = Self::default();
    let config_path = Self::filepath();

    let mut minimal_config = Map::new();

    if self.vault.path != default_config.vault.path {
      let mut vault_table = Map::new();
      vault_table.insert("path".to_string(), toml::Value::String(self.vault.path.to_string_lossy().to_string()));
      minimal_config.insert("vault".to_string(), toml::Value::Table(vault_table));
    }

    for (key, value) in &self.extensions {
      minimal_config.insert(key.clone(), value.clone());
    }

    if minimal_config.is_empty() {
      if config_path.exists() {
        std::fs::remove_file(&config_path)?;
      }
    } else {
      let content = toml::to_string_pretty(&minimal_config)?;
      std::fs::create_dir_all(config_path.parent().unwrap())?;
      std::fs::write(&config_path, content)?;
    }

    Ok(())
  }
}
