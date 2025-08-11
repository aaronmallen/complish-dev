use clap::Args;
use eyre::{Result, eyre};
use toml::map::Map;

use crate::config::Config;

/// Set a configuration value
#[derive(Args, Debug)]
pub struct Set {
  /// Configuration key to set
  pub key: String,
  /// Value to set
  pub value: String,
}

impl Set {
  pub fn run(&self) -> Result<()> {
    let mut config = Config::load();
    self.set_config_value(&mut config, &self.key, &self.value)?;

    config.save()?;

    Ok(())
  }

  fn set_config_value(&self, config: &mut Config, key: &str, value: &str) -> Result<()> {
    let keys: Vec<&str> = key.split('.').collect();

    if keys.is_empty() {
      return Err(eyre!("Empty key not allowed"));
    }

    if self.would_conflict_with_defaults(key)? {
      return Err(eyre!("Cannot set '{}' as a value. This key conflicts with default configuration structure.", key));
    }

    match keys.as_slice() {
      ["vault", "path"] => {
        config.vault.path = std::path::PathBuf::from(value);
      }
      _ => {
        let current_table = &mut config.extensions;

        if let Some(&key_part) = keys[..keys.len() - 1].iter().next() {
          let entry = current_table.entry(key_part.to_string()).or_insert_with(|| toml::Value::Table(Map::new()));

          if let toml::Value::Table(nested_table) = entry {
            return self.set_extension_value(config, key, value);
          }
          return Err(eyre!("Key '{}' already exists as non-table value", key_part));
        }

        let final_key = (*keys.last().unwrap()).to_string();
        let parsed_value = self.parse_value(value);
        current_table.insert(final_key, parsed_value);
      }
    }

    Ok(())
  }

  fn set_extension_value(&self, config: &mut Config, key: &str, value: &str) -> Result<()> {
    let keys: Vec<&str> = key.split('.').collect();

    let extensions_toml = toml::to_string(&config.extensions)?;
    let mut extensions_table: Map<String, toml::Value> = if extensions_toml.trim().is_empty() {
      Map::new()
    } else {
      toml::from_str(&extensions_toml)?
    };

    let mut current_table = &mut extensions_table;

    for &key_part in &keys[..keys.len() - 1] {
      let entry = current_table.entry(key_part.to_string()).or_insert_with(|| toml::Value::Table(Map::new()));

      if let toml::Value::Table(nested_table) = entry {
        current_table = nested_table;
      } else {
        return Err(eyre!("Key '{}' already exists as non-table value", key_part));
      }
    }

    let final_key = (*keys.last().unwrap()).to_string();
    if let Some(existing) = current_table.get(&final_key)
      && matches!(existing, toml::Value::Table(_))
    {
      return Err(eyre!("Cannot overwrite section '{}' with a value. Existing subsections would be lost.", key));
    }

    let parsed_value = self.parse_value(value);
    current_table.insert(final_key, parsed_value);

    config.extensions = extensions_table.into_iter().collect();

    Ok(())
  }

  fn would_conflict_with_defaults(&self, key: &str) -> Result<bool> {
    let default_config = Config::default();
    let default_toml = toml::to_string(&default_config)?;
    let default_table: Map<String, toml::Value> = toml::from_str(&default_toml)?;

    Ok(self.key_exists_as_table(&default_table, key))
  }

  fn key_exists_as_table(&self, table: &Map<String, toml::Value>, key: &str) -> bool {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current = table;

    for key_part in keys {
      match current.get(key_part) {
        Some(toml::Value::Table(nested_table)) => {
          current = nested_table;
        }
        Some(_) | None => {
          return false;
        }
      }
    }
    true
  }

  fn parse_value(&self, value: &str) -> toml::Value {
    if let Ok(bool_val) = value.parse::<bool>() {
      return toml::Value::Boolean(bool_val);
    }

    if let Ok(int_val) = value.parse::<i64>() {
      return toml::Value::Integer(int_val);
    }

    if let Ok(float_val) = value.parse::<f64>() {
      return toml::Value::Float(float_val);
    }

    toml::Value::String(value.to_string())
  }
}
