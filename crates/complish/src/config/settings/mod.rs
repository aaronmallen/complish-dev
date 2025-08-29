use std::path::PathBuf;

use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct Settings {
  #[get = "pub"]
  pub(crate) store_path: PathBuf,
}
