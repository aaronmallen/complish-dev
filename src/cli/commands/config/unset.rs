use clap::Args;
use eyre::{Result, eyre};
use toml::map::Map;

use crate::config::Config;

/// Unset a configuration value
#[derive(Args, Debug)]
pub struct Unset {
  /// Configuration key to unset
  pub key: String,
}

impl Unset {
  pub fn run(&self) -> Result<()> {
    let config_path = Config::filepath();

    if !config_path.exists() {
      return Err(eyre!("No configuration file found"));
    }

    let content = std::fs::read_to_string(&config_path)?;
    let mut toml_table: Map<String, toml::Value> = toml::from_str(&content)?;

    if self.unset_nested_value(&mut toml_table, &self.key)? {
      let toml_content = toml::to_string_pretty(&toml_table)?;
      std::fs::write(&config_path, toml_content)?;
      println!("Unset '{}'", self.key);
    } else {
      return Err(eyre!("Configuration key '{}' not found", self.key));
    }

    Ok(())
  }

  fn unset_nested_value(&self, table: &mut Map<String, toml::Value>, key: &str) -> Result<bool> {
    let keys: Vec<&str> = key.split('.').collect();

    if keys.is_empty() {
      return Err(eyre!("Empty key not allowed"));
    }

    if keys.len() == 1 {
      return Ok(table.remove(keys[0]).is_some());
    }

    if !self.key_exists(table, &keys) {
      return Ok(false);
    }

    let removed = Self::remove_key(table, &keys);

    if removed {
      self.cleanup_empty_tables(table, &keys[..keys.len() - 1]);
    }

    Ok(removed)
  }

  fn key_exists(&self, table: &Map<String, toml::Value>, keys: &[&str]) -> bool {
    let mut current = table;

    for &key_part in &keys[..keys.len() - 1] {
      match current.get(key_part) {
        Some(toml::Value::Table(nested_table)) => {
          current = nested_table;
        }
        _ => return false,
      }
    }

    current.contains_key(*keys.last().unwrap())
  }

  fn remove_key(table: &mut Map<String, toml::Value>, keys: &[&str]) -> bool {
    if keys.len() == 1 {
      return table.remove(keys[0]).is_some();
    }

    if let Some(toml::Value::Table(nested_table)) = table.get_mut(keys[0]) {
      return Self::remove_key(nested_table, &keys[1..]);
    }

    false
  }

  fn cleanup_empty_tables(&self, table: &mut Map<String, toml::Value>, path: &[&str]) {
    if path.is_empty() {
      return;
    }

    if self.is_nested_table_empty(table, path) {
      Self::remove_nested_table(table, path);
      if path.len() > 1 {
        self.cleanup_empty_tables(table, &path[..path.len() - 1]);
      }
    }
  }

  fn is_nested_table_empty(&self, table: &Map<String, toml::Value>, path: &[&str]) -> bool {
    let mut current = table;

    for &key_part in path {
      match current.get(key_part) {
        Some(toml::Value::Table(nested_table)) => {
          current = nested_table;
        }
        _ => return false,
      }
    }

    current.is_empty()
  }

  fn remove_nested_table(table: &mut Map<String, toml::Value>, path: &[&str]) {
    if path.is_empty() {
      return;
    }

    if path.len() == 1 {
      table.remove(path[0]);
      return;
    }

    if let Some(toml::Value::Table(nested_table)) = table.get_mut(path[0]) {
      Self::remove_nested_table(nested_table, &path[1..]);
    }
  }
}
