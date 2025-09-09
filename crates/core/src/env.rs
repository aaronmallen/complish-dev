use std::{env::var_os, ffi::OsString, path::PathBuf, sync::LazyLock};

pub static COMPLISH_DATA_DIR: LazyLock<Value<PathBuf>> = LazyLock::new(|| {
  var_os("COMPLISH_DATA_DIR")
    .map(PathBuf::from)
    .map_or(Value::NotSet, |path| {
      if path.is_absolute() {
        Value::Ok(path)
      } else {
        Value::Err(path.into_os_string())
      }
    })
});

pub enum Value<T> {
  Err(OsString),
  NotSet,
  Ok(T),
}
