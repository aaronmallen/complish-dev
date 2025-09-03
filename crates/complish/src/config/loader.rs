use std::{fs, path::Path};

use eyre::Result;
use getset::Getters;

use super::settings::Settings;
use crate::{env, env::Value as EnvValue};

#[derive(Debug, Getters)]
pub struct Config {
  #[get = "pub"]
  settings: Settings,
}

impl Config {
  pub fn load() -> Result<Self> {
    let config_home = if let EnvValue::Ok(path) = env::COMPLISH_CONFIG_HOME.value() {
      path.to_owned()
    } else {
      super::default_path::config_home()?
    };

    Self::load_from(&config_home)
  }

  pub fn load_from(config_home: &Path) -> Result<Self> {
    let config_file = super::default_path::config_file(config_home);
    let settings = if config_file.exists() {
      toml::from_str::<Settings>(&fs::read_to_string(config_file)?)?
    } else {
      Settings::default()
    };

    Ok(Self {
      settings,
    })
  }
}
