use std::{ffi::OsString, path::PathBuf};

use super::Value;

pub fn resolve_absolute_path(var: Option<OsString>) -> Value<PathBuf> {
  var.map(PathBuf::from).map_or(Value::NotSet, |path| {
    if path.is_absolute() {
      Value::Ok(path)
    } else {
      Value::Err(path.into())
    }
  })
}
