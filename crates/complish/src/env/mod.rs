mod value;
mod var;

use std::{ffi::OsString, fmt::Debug, path::PathBuf, sync::LazyLock};

pub use value::Value;
pub use var::Var;

use crate::diagnostic::{Set as DiagnosticSet, Severity as DiagnosticSeverity};

pub static COMPLISH_CONFIG_HOME: LazyLock<Var<PathBuf, PathBuf>> =
  LazyLock::new(|| Var::new("COMPLISH_CONFIG_HOME", resolve_path));

pub static COMPLISH_DATA_HOME: LazyLock<Var<PathBuf, PathBuf>> =
  LazyLock::new(|| Var::new("COMPLISH_DATA_HOME", resolve_path));

pub fn diagnostic() -> DiagnosticSet {
  let mut set = DiagnosticSet::new();

  check_var(&mut set, &COMPLISH_CONFIG_HOME, "is not an absolute path");
  check_var(&mut set, &COMPLISH_DATA_HOME, "is not an absolute path");

  set
}

fn check_var<T, E>(set: &mut DiagnosticSet, var: &Var<T, E>, message: &str)
where
  E: Debug,
{
  if let Value::Invalid(val) = var.value() {
    set.add(
      DiagnosticSeverity::Warn,
      format!("{message}: {val:?}"),
      var.key(),
    );
  }
}

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

  mod diagnostic {
    use pretty_assertions::assert_eq;
    use temp_env::with_vars;

    use super::*;

    #[test]
    fn it_returns_diagnostics_when_config_home_is_not_absolute() {
      let relative_path = "test/path";
      let absolute_path = "/test/path";

      with_vars(
        [
          ("COMPLISH_CONFIG_HOME", Some(relative_path)),
          ("COMPLISH_DATA_HOME", Some(absolute_path)),
        ],
        || {
          let diagnostic = diagnostic();

          assert_eq!(diagnostic.warnings().len(), 1);
          assert_eq!(diagnostic[0].context(), "COMPLISH_CONFIG_HOME");
          assert_eq!(
            diagnostic[0].message(),
            "is not an absolute path: \"test/path\""
          );
        },
      );
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
