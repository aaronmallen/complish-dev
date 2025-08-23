use std::{env, path::PathBuf};

mod complish_home_result;

pub use complish_home_result::ComplishHomeResult;

const COMPLISH_HOME_KEY: &str = "COMPLISH_HOME";

pub fn complish_home() -> ComplishHomeResult {
  match env::var(COMPLISH_HOME_KEY) {
    Ok(home) => {
      let path = PathBuf::from(home);
      if path.is_absolute() {
        ComplishHomeResult::Found(path)
      } else {
        ComplishHomeResult::NotAbsolute(path)
      }
    }
    Err(_) => ComplishHomeResult::NotSet,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod complish_home {
    use pretty_assertions::assert_eq;
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn it_returns_found_when_complish_home_is_set_to_an_absolute_path() {
      let path = "/test/path";
      let expected = ComplishHomeResult::Found(PathBuf::from(path));

      with_var(COMPLISH_HOME_KEY, Some(path), || {
        assert_eq!(complish_home(), expected);
      });
    }

    #[test]
    fn it_returns_not_absolute_when_complish_home_is_set_to_a_relative_path() {
      let path = "test/path";
      let expected = ComplishHomeResult::NotAbsolute(PathBuf::from(path));

      with_var(COMPLISH_HOME_KEY, Some(path), || {
        assert_eq!(complish_home(), expected);
      });
    }

    #[test]
    fn it_returns_not_set_when_complish_home_is_not_set() {
      with_var_unset(COMPLISH_HOME_KEY, || {
        assert_eq!(complish_home(), ComplishHomeResult::NotSet);
      });
    }
  }
}
