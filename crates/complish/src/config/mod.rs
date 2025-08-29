pub mod env;
pub mod path;
mod settings;

use std::{
  fs,
  path::{Path, PathBuf},
};

use eyre::Result;
use getset::Getters;
use settings::Settings;

#[derive(Clone, Debug, Eq, Getters, PartialEq)]
pub struct Config {
  #[get = "pub"]
  path: PathBuf,
  #[get = "pub"]
  settings: Settings,
}

impl Config {
  pub fn load(config_path: &Path) -> Result<Self> {
    let settings = if config_path.exists() {
      toml::from_str::<Settings>(&fs::read_to_string(config_path)?)?
    } else {
      Settings {
        store_path: path::default_store_file()?,
      }
    };

    Ok(Self {
      path: config_path.to_owned(),
      settings,
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod load {
    use pretty_assertions::assert_eq;
    use tempdir::TempDir;

    use super::*;

    #[test]
    fn it_loads_from_file_if_file_exists() {
      let temp = TempDir::new("complish").unwrap();
      let config_path = temp.path().join("config");
      let mock_config = "store_path = \"/some/fake/path\"\n";
      fs::write(&config_path, mock_config).unwrap();
      let config = Config::load(&config_path).unwrap();

      assert_eq!(
        config.settings.store_path(),
        &PathBuf::from("/some/fake/path")
      );
    }

    #[test]
    fn it_loads_defaults_when_file_does_not_exist() {
      let temp = TempDir::new("complish").unwrap();
      let config_path = temp.path().join("config");
      let config = Config::load(&config_path).unwrap();

      assert_eq!(
        config.settings.store_path(),
        &path::default_store_file().unwrap()
      );
    }
  }
}
