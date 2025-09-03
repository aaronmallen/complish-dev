use std::path::Path;

use super::loader::*;
use crate::diagnostic::{Set as DiagnosticSet, Severity as DiagnosticSeverity};

impl Config {
  pub fn diagnostic(&self) -> DiagnosticSet {
    let mut set = DiagnosticSet::new();

    Self::check_path_setting(
      &mut set,
      self.settings().core().data_home(),
      "core.data_home",
    );

    set
  }

  fn check_path_setting(set: &mut DiagnosticSet, setting: &Path, key: impl Into<String>) {
    if !setting.is_absolute() {
      set.add(
        DiagnosticSeverity::Warn,
        format!("is not an absolute path: \"{}\"", setting.display()),
        key.into(),
      )
    }
  }
}
