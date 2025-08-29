use std::path::PathBuf;

use eyre::{Result, eyre};

pub fn default_config_file() -> Result<PathBuf> {
  dir_spec::config_home()
    .map(|p| p.join("complish/config"))
    .ok_or_else(|| eyre!("Could not find user's config directory"))
}

pub fn default_store_file() -> Result<PathBuf> {
  dir_spec::data_home()
    .map(|p| p.join("complish/store"))
    .ok_or_else(|| eyre!("Could not find user's data directory"))
}

#[cfg(test)]
mod test {
  use super::*;

  mod default_config_file {
    use pretty_assertions::assert_eq;
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_returns_the_default_config_file_path() {
      let xdg_config_home = "/.config";

      with_var("XDG_CONFIG_HOME", Some(xdg_config_home), || {
        assert_eq!(
          default_config_file().unwrap(),
          PathBuf::from(xdg_config_home).join("complish/config")
        );
      });
    }
  }

  mod default_store_file {
    use pretty_assertions::assert_eq;
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_returns_the_default_store_file_path() {
      let xdg_data_home = "/.local/share";

      with_var("XDG_DATA_HOME", Some(xdg_data_home), || {
        assert_eq!(
          default_store_file().unwrap(),
          PathBuf::from(xdg_data_home).join("complish/store")
        );
      });
    }
  }
}
