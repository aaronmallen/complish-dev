use clap::Args;
use eyre::{Result, eyre};

use crate::config::Config;

/// Get a configuration value
#[derive(Args, Debug)]
pub struct Get {
  /// Configuration key to retrieve
  pub key: Option<String>,
}

impl Get {
  pub fn run(&self) -> Result<()> {
    let config = Config::load();

    match &self.key {
      Some(key) => {
        if let Some(value) = self.get_from_file(key)? {
          println!("{value}");
          return Ok(());
        }

        if let Some(value) = self.get_from_config(&config, key)? {
          println!("{value}");
          Ok(())
        } else {
          println!("Configuration key '{key}' not found");
          Ok(())
        }
      }

      None => {
        let config_toml = toml::to_string_pretty(&config)?;
        println!("{config_toml}");
        Ok(())
      }
    }
  }

  fn get_from_file(&self, key: &str) -> Result<Option<String>> {
    let config_path = Config::filepath();
    if !config_path.exists() {
      return Ok(None);
    }

    let content = std::fs::read_to_string(config_path)?;
    let toml_value: toml::Value = toml::from_str(&content)?;

    Ok(self.get_nested_value(&toml_value, key))
  }

  fn get_from_config(&self, config: &Config, key: &str) -> Result<Option<String>> {
    let config_toml = toml::to_string(config)?;
    let toml_value: toml::Value = toml::from_str(&config_toml)?;

    Ok(self.get_nested_value(&toml_value, key))
  }

  fn get_nested_value(&self, value: &toml::Value, key: &str) -> Option<String> {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current = value;

    for key_part in keys {
      if let toml::Value::Table(table) = current {
        current = table.get(key_part)?;
      } else {
        return None;
      }
    }

    Some(current.to_string())
  }
}
