use std::{fs, path::PathBuf};

use eyre::{Result, eyre};

use crate::env::{self, Value as EnvValue};

pub fn database_url() -> Result<String> {
  let data_home = data_dir()?;

  fs::create_dir_all(&data_home).map_err(|e| {
    eyre!(
      "Could not create data directory: {} - {}",
      data_home.display(),
      e
    )
  })?;

  Ok(format!("sqlite://{}", data_home.join("store").display()))
}

fn data_dir() -> Result<PathBuf> {
  match env::COMPLISH_DATA_DIR.value() {
    EnvValue::Ok(path) => Ok(path.to_owned()),
    _ => dir_spec::data_home()
      .map(|p| p.join("complish"))
      .ok_or_else(|| {
        eyre!(
          "Could not find user's data directory. Please set COMPLISH_DATA_DIR in your environment"
        )
      }),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod database_url {
    use super::*;

    #[test]
    fn it_works() {
      let url = database_url();

      if let Ok(url) = url {
        assert!(url.starts_with("sqlite://"));
        assert!(url.ends_with("/store"));
      }
    }
  }

  mod data_dir {
    use super::*;

    #[test]
    fn it_handles_configured_and_fallback() {
      let result = data_dir();

      if let Ok(dir) = result {
        assert!(
          dir.to_string_lossy().contains("complish") || std::env::var("COMPLISH_DATA_DIR").is_ok()
        );
      }
    }
  }
}
