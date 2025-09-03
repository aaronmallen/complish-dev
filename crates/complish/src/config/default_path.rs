use std::path::{Path, PathBuf};

use eyre::{Result, eyre};

pub fn config_home() -> Result<PathBuf> {
  dir_spec::config_home()
    .map(|p| p.join("complish"))
    .ok_or_else(|| eyre!("Could not find user's config directory"))
}

pub fn data_home() -> Result<PathBuf> {
  dir_spec::data_home()
    .map(|p| p.join("complish"))
    .ok_or_else(|| eyre!("Could not find user's data directory"))
}

pub fn config_file(config_home: &Path) -> PathBuf {
  config_home.join("config")
}

pub fn store_file(data_home: &Path) -> PathBuf {
  data_home.join("store")
}

#[cfg(test)]
mod test {
  use super::*;

  mod config_home {
    use pretty_assertions::assert_eq;
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_returns_the_default_config_dir() {
      let xdg_config_home = "/.config";

      with_var("XDG_CONFIG_HOME", Some(xdg_config_home), || {
        assert_eq!(
          config_home().unwrap(),
          Path::new(xdg_config_home).join("complish")
        );
      });
    }
  }

  mod data_home {
    use pretty_assertions::assert_eq;
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_returns_the_default_data_dir() {
      let xdg_data_home = "/.local/share";

      with_var("XDG_DATA_HOME", Some(xdg_data_home), || {
        assert_eq!(
          data_home().unwrap(),
          PathBuf::from(xdg_data_home).join("complish")
        );
      });
    }
  }

  mod config_file {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_the_config_file_path() {
      let config_home = config_home().unwrap();
      assert_eq!(config_file(&config_home), config_home.join("config"));
    }
  }

  mod store_file {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_the_store_file_path() {
      let data_home = data_home().unwrap();
      assert_eq!(store_file(&data_home), data_home.join("store"));
    }
  }
}
