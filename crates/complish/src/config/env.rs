use std::{env, path::PathBuf, sync::LazyLock};

use crate::types::EnvResult;

pub static COMPLISH_CONFIG: LazyLock<EnvResult<PathBuf, PathBuf>> =
  LazyLock::new(|| resolve_path("COMPLISH_CONFIG"));

fn resolve_path(key: &str) -> EnvResult<PathBuf, PathBuf> {
  env::var_os(key)
    .map(PathBuf::from)
    .map_or(EnvResult::NotSet, |path| {
      if path.is_absolute() {
        EnvResult::Ok(path)
      } else {
        EnvResult::Invalid(path)
      }
    })
}

#[cfg(test)]
mod test {
  use super::*;

  mod complish_config {
    use pretty_assertions::assert_eq;
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn it_returns_ok_when_set_to_an_absolute_path() {
      let path = "/test/path";

      with_var("COMPLISH_CONFIG", Some(path), || {
        assert_eq!(
          resolve_path("COMPLISH_CONFIG"),
          EnvResult::Ok(PathBuf::from(path))
        );
      });
    }

    #[test]
    fn it_returns_invalid_when_set_to_a_relative_path() {
      let path = "test/path";

      with_var("COMPLISH_CONFIG", Some(path), || {
        assert_eq!(
          resolve_path("COMPLISH_CONFIG"),
          EnvResult::Invalid(PathBuf::from(path))
        );
      });
    }

    #[test]
    fn it_returns_not_set_when_unset() {
      with_var_unset("COMPLISH_CONFIG", || {
        assert_eq!(resolve_path("COMPLISH_CONFIG"), EnvResult::NotSet);
      });
    }
  }
}
