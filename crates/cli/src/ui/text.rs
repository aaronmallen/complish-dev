use yansi::Paint;

use super::color::*;

pub fn error(string: impl AsRef<str>) -> String {
  string.as_ref().fg(ERROR_COLOR).to_string()
}

pub fn info(string: impl AsRef<str>) -> String {
  string.as_ref().fg(INFO_COLOR).to_string()
}

pub fn success(string: impl AsRef<str>) -> String {
  string.as_ref().fg(SUCCESS_COLOR).to_string()
}

pub fn warn(string: impl AsRef<str>) -> String {
  string.as_ref().fg(WARN_COLOR).to_string()
}
