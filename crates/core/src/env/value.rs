use std::ffi::OsString;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value<T> {
  Err(OsString),
  NotSet,
  Ok(T),
}
