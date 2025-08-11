use std::path::PathBuf;

use dir_spec::Dir;

pub struct FilePath;

impl FilePath {
  pub fn resolve() -> PathBuf {
    if let Ok(config_path) = std::env::var("COMPLISH_CONFIG") {
      return PathBuf::from(config_path);
    }

    let locations = [
      Dir::config_home().map(|p| p.join("complish/config.toml")),
      Dir::home().map(|p| p.join(".config/complish/config.toml")),
      Dir::home().map(|p| p.join("complishrc")),
    ];

    for location in locations.iter().flatten() {
      if location.exists() {
        return location.clone();
      }
    }

    Dir::config_home().map_or_else(|| PathBuf::from("complish/config.toml"), |p| p.join("complish/config.toml"))
  }
}
