use std::{env::var_os, ffi::OsString};

use getset::Getters;

use super::Value;

#[derive(Clone, Debug, Eq, Getters, PartialEq)]
pub struct Var<T> {
  #[get = "pub"]
  key: String,
  #[get = "pub"]
  value: Value<T>,
}

impl<T> Var<T> {
  pub fn new(key: impl Into<String>, resolver: fn(Option<OsString>) -> Value<T>) -> Self {
    let key = key.into();
    let value = resolver(var_os(&key));

    Self {
      key,
      value,
    }
  }
}
