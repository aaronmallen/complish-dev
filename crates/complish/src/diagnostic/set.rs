use std::{
  ops::{Deref, DerefMut},
  vec,
};

use super::{Result, Severity};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Set(Vec<Result>);

impl Set {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add(
    &mut self,
    severity: Severity,
    message: impl Into<String>,
    context: impl Into<String>,
  ) {
    self.0.push(Result::new(severity, message, context));
  }

  pub fn errors(&self) -> Self {
    self.having_severity(&Severity::Error)
  }

  pub fn has_errors(&self) -> bool {
    self.has_severity(&Severity::Error)
  }

  pub fn has_infos(&self) -> bool {
    self.has_severity(&Severity::Info)
  }

  pub fn has_warnings(&self) -> bool {
    self.has_severity(&Severity::Warn)
  }

  pub fn infos(&self) -> Self {
    self.having_severity(&Severity::Info)
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn map<F, T>(&self, f: F) -> Vec<T>
  where
    F: Fn(&Result) -> T,
  {
    self.0.iter().map(f).collect()
  }

  pub fn push(&mut self, result: Result) {
    self.0.push(result);
  }

  pub fn warnings(&self) -> Self {
    self.having_severity(&Severity::Warn)
  }

  fn has_severity(&self, severity: &Severity) -> bool {
    self.iter().any(|d| d.severity() == severity)
  }

  fn having_severity(&self, severity: &Severity) -> Self {
    let filtered: Vec<Result> = self
      .0
      .iter()
      .filter(|d| d.severity() == severity)
      .cloned()
      .collect();
    Self::from(filtered)
  }
}

impl Deref for Set {
  type Target = Vec<Result>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Set {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Extend<Result> for Set {
  fn extend<T: IntoIterator<Item = Result>>(&mut self, iter: T) {
    self.0.extend(iter);
  }
}

impl Extend<Set> for Set {
  fn extend<T: IntoIterator<Item = Set>>(&mut self, iter: T) {
    for set in iter {
      self.0.extend(set.0);
    }
  }
}

impl From<Vec<Result>> for Set {
  fn from(results: Vec<Result>) -> Self {
    Self(results)
  }
}

impl From<Set> for Vec<Result> {
  fn from(set: Set) -> Self {
    set.0
  }
}

impl IntoIterator for Set {
  type IntoIter = vec::IntoIter<Result>;
  type Item = Result;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a Set {
  type IntoIter = std::slice::Iter<'a, Result>;
  type Item = &'a Result;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<'a> IntoIterator for &'a mut Set {
  type IntoIter = std::slice::IterMut<'a, Result>;
  type Item = &'a mut Result;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter_mut()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod add {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_a_result() {
      let mut set = Set::new();
      set.add(Severity::Error, "test error", "test context");

      assert_eq!(set.len(), 1);
      assert_eq!(set[0].severity(), &Severity::Error);
      assert_eq!(set[0].message(), "test error");
      assert_eq!(set[0].context(), "test context");
    }
  }

  mod errors {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_set_with_only_errors() {
      let mut set = Set::new();
      set.add(Severity::Error, "test error", "test context");
      set.add(Severity::Warn, "test warn", "test context");
      set.add(Severity::Info, "test info", "test context");
      let errors = set.errors();

      assert_eq!(errors.len(), 1);
      assert!(errors.iter().all(|d| d.severity() == &Severity::Error));
    }

    #[test]
    fn it_returns_an_empty_set_if_there_are_no_errors() {
      let mut set = Set::new();
      set.add(Severity::Warn, "test warn", "test context");
      set.add(Severity::Info, "test info", "test context");
      let errors = set.errors();

      assert!(errors.is_empty());
    }
  }

  mod has_errors {
    use super::*;

    #[test]
    fn it_returns_true_if_there_are_errors() {
      let mut set = Set::new();
      set.add(Severity::Error, "test error", "test context");

      assert!(set.has_errors());
    }

    #[test]
    fn it_returns_false_if_there_are_no_errors() {
      let mut set = Set::new();
      set.add(Severity::Warn, "test warn", "test context");

      assert!(!set.has_errors());
    }
  }

  mod has_infos {
    use super::*;

    #[test]
    fn it_returns_true_if_there_are_infos() {
      let mut set = Set::new();
      set.add(Severity::Info, "test info", "test context");

      assert!(set.has_infos());
    }

    #[test]
    fn it_returns_false_if_there_are_no_infos() {
      let mut set = Set::new();
      set.add(Severity::Warn, "test warn", "test context");

      assert!(!set.has_infos());
    }
  }

  mod has_warnings {
    use super::*;

    #[test]
    fn it_returns_true_if_there_are_warnings() {
      let mut set = Set::new();
      set.add(Severity::Warn, "test warn", "test context");

      assert!(set.has_warnings());
    }

    #[test]
    fn it_returns_false_if_there_are_no_warnings() {
      let mut set = Set::new();
      set.add(Severity::Info, "test info", "test context");

      assert!(!set.has_warnings());
    }
  }

  mod infos {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_set_with_only_infos() {
      let mut set = Set::new();
      set.add(Severity::Error, "test error", "test context");
      set.add(Severity::Warn, "test warn", "test context");
      set.add(Severity::Info, "test info", "test context");
      let infos = set.infos();

      assert_eq!(infos.len(), 1);
      assert!(infos.iter().all(|d| d.severity() == &Severity::Info));
    }

    #[test]
    fn it_returns_an_empty_set_if_there_are_no_infos() {
      let mut set = Set::new();
      set.add(Severity::Error, "test error", "test context");
      set.add(Severity::Warn, "test warn", "test context");
      let infos = set.infos();

      assert!(infos.is_empty());
    }
  }

  mod is_empty {
    use super::*;

    #[test]
    fn it_returns_true_if_the_set_is_empty() {
      let set = Set::new();

      assert!(set.is_empty());
    }

    #[test]
    fn it_returns_false_if_the_set_is_not_empty() {
      let mut set = Set::new();
      set.add(Severity::Error, "test error", "test context");

      assert!(!set.is_empty());
    }
  }

  mod len {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_the_number_of_results() {
      let mut set = Set::new();
      set.add(Severity::Error, "test error", "test context");
      set.add(Severity::Warn, "test warn", "test context");
      set.add(Severity::Info, "test info", "test context");

      assert_eq!(set.len(), 3);
    }
  }

  mod map {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_maps_results_to_messages() {
      let mut set = Set::new();
      set.add(Severity::Error, "error message", "error context");
      set.add(Severity::Warn, "warn message", "warn context");
      set.add(Severity::Info, "info message", "info context");

      let messages = set.map(|r| r.message().clone());

      assert_eq!(
        messages,
        vec!["error message", "warn message", "info message"]
      );
    }

    #[test]
    fn it_maps_results_to_contexts() {
      let mut set = Set::new();
      set.add(Severity::Error, "error message", "error context");
      set.add(Severity::Warn, "warn message", "warn context");

      let contexts = set.map(|r| r.context().clone());

      assert_eq!(contexts, vec!["error context", "warn context"]);
    }

    #[test]
    fn it_maps_results_to_severities() {
      let mut set = Set::new();
      set.add(Severity::Error, "error message", "error context");
      set.add(Severity::Info, "info message", "info context");

      let severities = set.map(|r| r.severity().clone());

      assert_eq!(severities, vec![Severity::Error, Severity::Info]);
    }

    #[test]
    fn it_returns_empty_vec_for_empty_set() {
      let set = Set::new();
      let messages: Vec<String> = set.map(|r| r.message().clone());

      assert!(messages.is_empty());
    }
  }

  mod push {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_a_result() {
      let mut set = Set::new();
      set.push(Result::new(Severity::Error, "test error", "test context"));

      assert_eq!(set.len(), 1);
      assert_eq!(set[0].severity(), &Severity::Error);
      assert_eq!(set[0].message(), "test error");
      assert_eq!(set[0].context(), "test context");
    }
  }

  mod warnings {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_set_with_only_warns() {
      let mut set = Set::new();
      set.add(Severity::Error, "test error", "test context");
      set.add(Severity::Warn, "test warn", "test context");
      set.add(Severity::Info, "test info", "test context");
      let warnings = set.warnings();

      assert_eq!(warnings.len(), 1);
      assert!(warnings.iter().all(|d| d.severity() == &Severity::Warn));
    }

    #[test]
    fn it_returns_an_empty_set_if_there_are_no_warns() {
      let mut set = Set::new();
      set.add(Severity::Error, "test error", "test context");
      set.add(Severity::Info, "test info", "test context");
      let warnings = set.warnings();

      assert!(warnings.is_empty());
    }
  }
}
