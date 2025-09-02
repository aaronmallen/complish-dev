mod value;
mod var;

use std::{ffi::OsString, path::PathBuf, sync::LazyLock};

pub use value::Value;
pub use var::Var;

pub static COMPLISH_CONFIG_HOME: LazyLock<Var<PathBuf, PathBuf>> =
  LazyLock::new(|| Var::new("COMPLISH_CONFIG_HOME", resolve_path));

pub static COMPLISH_DATA_HOME: LazyLock<Var<PathBuf, PathBuf>> =
  LazyLock::new(|| Var::new("COMPLISH_DATA_HOME", resolve_path));

fn resolve_path(var: Option<OsString>) -> Value<PathBuf, PathBuf> {
  var.map(PathBuf::from).map_or(Value::NotSet, |path| {
    if path.is_absolute() {
      Value::Ok(path)
    } else {
      Value::Invalid(path)
    }
  })
}

#[cfg(test)]
mod test {
  use super::*;

  #[allow(non_snake_case)]
  mod COMPLISH_CONFIG_HOME {
    use pretty_assertions::assert_eq;
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_returns_an_env_var() {
      let path = "/test/path";

      with_var("COMPLISH_CONFIG_HOME", Some(path), || {
        assert_eq!(COMPLISH_CONFIG_HOME.key(), &"COMPLISH_CONFIG_HOME");
        assert_eq!(
          COMPLISH_CONFIG_HOME.value(),
          &Value::Ok(PathBuf::from(path))
        );
      });
    }
  }

  #[allow(non_snake_case)]
  mod COMPLISH_DATA_HOME {
    use pretty_assertions::assert_eq;
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_returns_an_env_var() {
      let path = "/test/path";

      with_var("COMPLISH_DATA_HOME", Some(path), || {
        assert_eq!(COMPLISH_DATA_HOME.key(), &"COMPLISH_DATA_HOME");
        assert_eq!(COMPLISH_DATA_HOME.value(), &Value::Ok(PathBuf::from(path)));
      });
    }
  }

  mod resolve_path {
    use pretty_assertions::assert_eq;
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn it_returns_ok_when_set_to_absolute_path() {
      let path = "/test/path";

      with_var("COMPLISH_TEST_VAR", Some(path), || {
        assert_eq!(
          resolve_path(Some(OsString::from(path))),
          Value::Ok(PathBuf::from(path))
        );
      });
    }

    #[test]
    fn it_returns_invalid_when_set_to_relative_path() {
      let path = "test/path";

      with_var("COMPLISH_TEST_VAR", Some(path), || {
        assert_eq!(
          resolve_path(Some(OsString::from(path))),
          Value::Invalid(PathBuf::from(path))
        );
      });
    }

    #[test]
    fn it_returns_not_set_when_unset() {
      with_var_unset("COMPLISH_TEST_VAR", || {
        assert_eq!(resolve_path(None), Value::NotSet);
      });
    }
  }
}
