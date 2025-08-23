use std::path::PathBuf;

use dir_spec::Dir;
use eyre::{Result, eyre};

pub struct Path {
  home: PathBuf,
}

impl Path {
  pub fn new(home: PathBuf) -> Self {
    Self {
      home,
    }
  }

  pub fn default_home_dir() -> Result<PathBuf> {
    Dir::data_home()
      .map(|dir| dir.join("complish"))
      .ok_or_else(|| eyre!("Could not find user's data directory"))
  }

  pub fn home_dir(&self) -> PathBuf {
    self.home.clone()
  }

  pub fn vault_file(&self) -> PathBuf {
    self.home.join("vault")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod default_home_dir {
    use pretty_assertions::assert_eq;
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_returns_the_default_home_directory() {
      let xdg_data_home = "/fake/data/dir";
      with_var("XDG_DATA_HOME", Some(xdg_data_home), || {
        let expected = PathBuf::from(xdg_data_home).join("complish");

        assert_eq!(Path::default_home_dir().unwrap(), expected);
      });
    }
  }

  mod home_dir {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_the_home_directory() {
      let home = PathBuf::from("/fake/home/directory");
      let path = Path::new(home.clone());

      assert_eq!(path.home_dir(), home);
    }
  }

  mod vault_file {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_the_vault_directory() {
      let home = PathBuf::from("/fake/home/directory");
      let path = Path::new(home.clone());

      assert_eq!(path.vault_file(), home.join("vault"));
    }
  }
}
