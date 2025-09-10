use std::{
  collections::HashMap,
  ops::{Deref, DerefMut},
};

use diesel::{
  backend::Backend,
  deserialize::{FromSql, FromSqlRow, Result as DeserializeResult},
  expression::AsExpression,
  serialize::{IsNull, Output, Result as SerializeResult, ToSql},
  sql_types::Text,
  sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

#[derive(
  Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, AsExpression, FromSqlRow,
)]
#[diesel(sql_type = Text)]
pub struct Metadata(HashMap<String, String>);

impl Metadata {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn get(&self, key: impl Into<String>) -> Option<&String> {
    self.0.get(&key.into())
  }

  pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
    self.0.insert(key.into(), value.into());
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn remove(&mut self, key: impl Into<String>) -> Option<String> {
    self.0.remove(&key.into())
  }
}

impl Deref for Metadata {
  type Target = HashMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Metadata {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl FromSql<Text, Sqlite> for Metadata {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> DeserializeResult<Self> {
    let s = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    serde_json::from_str(&s).map_err(Into::into)
  }
}

impl ToSql<Text, Sqlite> for Metadata {
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerializeResult {
    let json = serde_json::to_string(&self.0)?;
    out.set_value(json);
    Ok(IsNull::No)
  }
}
