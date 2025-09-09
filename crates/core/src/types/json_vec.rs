use std::ops::{Deref, DerefMut};

use diesel::{
  deserialize::{self, FromSql, FromSqlRow},
  expression::AsExpression,
  serialize::{self, IsNull, Output, ToSql},
  sql_types::Text,
  sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct JsonVec<T>(Vec<T>)
where
  T: Clone + std::fmt::Debug + PartialEq + Eq;

impl<T> Default for JsonVec<T>
where
  T: Clone + std::fmt::Debug + PartialEq + Eq,
{
  fn default() -> Self {
    Self(Vec::new())
  }
}

impl<T> JsonVec<T>
where
  T: Clone + std::fmt::Debug + PartialEq + Eq,
{
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self(Vec::with_capacity(capacity))
  }

  pub fn from_vec(vec: Vec<T>) -> Self {
    Self(vec)
  }

  pub fn into_vec(self) -> Vec<T> {
    self.0
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }
}

impl<T> Deref for JsonVec<T>
where
  T: Clone + std::fmt::Debug + PartialEq + Eq,
{
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for JsonVec<T>
where
  T: Clone + std::fmt::Debug + PartialEq + Eq,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> FromSql<Text, Sqlite> for JsonVec<T>
where
  T: for<'de> Deserialize<'de> + Clone + std::fmt::Debug + PartialEq + Eq,
{
  fn from_sql(
    bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
  ) -> deserialize::Result<Self> {
    let s = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    let vec: Vec<T> = serde_json::from_str(&s)
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
    Ok(JsonVec(vec))
  }
}

impl<T> ToSql<Text, Sqlite> for JsonVec<T>
where
  T: Serialize + Clone + std::fmt::Debug + PartialEq + Eq,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
    let json = serde_json::to_string(&self.0)?;
    out.set_value(json);
    Ok(IsNull::No)
  }
}

impl<T> From<Vec<T>> for JsonVec<T>
where
  T: Clone + std::fmt::Debug + PartialEq + Eq,
{
  fn from(vec: Vec<T>) -> Self {
    Self(vec)
  }
}

impl<T> From<JsonVec<T>> for Vec<T>
where
  T: Clone + std::fmt::Debug + PartialEq + Eq,
{
  fn from(json_vec: JsonVec<T>) -> Self {
    json_vec.0
  }
}
