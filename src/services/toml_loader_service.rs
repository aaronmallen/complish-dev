use std::path::Path;

use serde::de::DeserializeOwned;

pub struct TomlLoaderService;

impl TomlLoaderService {
  pub fn load_or_default<T>(path: &Path) -> T
  where
    T: DeserializeOwned + Default,
  {
    if path.exists()
      && let Ok(content) = std::fs::read_to_string(path)
      && let Ok(parsed) = toml::from_str::<T>(&content)
    {
      return parsed;
    }
    T::default()
  }
}
