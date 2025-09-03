use getset::Getters;
use serde::{Deserialize, Serialize};

use super::Core;

#[derive(Clone, Debug, Default, Deserialize, Getters, Serialize)]
pub struct Settings {
  #[get = "pub"]
  core: Core,
}
