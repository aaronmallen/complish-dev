use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComplishHomeResult {
  Found(PathBuf),
  NotAbsolute(PathBuf),
  NotSet,
}
