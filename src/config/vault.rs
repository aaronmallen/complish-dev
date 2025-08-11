use std::path::PathBuf;

use dir_spec::Dir;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vault {
  pub path: PathBuf,
}

impl Default for Vault {
  fn default() -> Self {
    Self {
      path: Dir::data_home().unwrap().join("complish"),
    }
  }
}
