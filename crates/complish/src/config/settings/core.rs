use std::path::PathBuf;

use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::{config::default_path, env, env::Value as EnvValue};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct Core {
  #[get = "pub"]
  data_home: PathBuf,
}

impl Core {
  pub fn store_file(&self) -> PathBuf {
    if self.data_home.is_absolute() {
      default_path::store_file(&self.data_home)
    } else {
      let data_home = default_path::data_home().unwrap_or(PathBuf::from("."));
      default_path::store_file(&data_home)
    }
  }
}

impl Default for Core {
  fn default() -> Self {
    let data_home = if let EnvValue::Ok(path) = env::COMPLISH_DATA_HOME.value() {
      path.to_owned()
    } else {
      default_path::data_home().unwrap_or(PathBuf::from("."))
    };

    Self {
      data_home,
    }
  }
}
