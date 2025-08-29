use std::ops::{Deref, DerefMut};

use rusqlite::{
  Error as SqliteError, Result as SqliteResult,
  types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct JsonVec<T>(pub Vec<T>);

impl<T> JsonVec<T> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn clear(&mut self) {
    self.0.clear();
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn push(&mut self, item: T) {
    self.0.push(item);
  }

  pub fn retain(&mut self, f: impl FnMut(&T) -> bool) {
    self.0.retain(f);
  }
}

impl<T> Default for JsonVec<T> {
  fn default() -> Self {
    Self(Vec::new())
  }
}

impl<T> Deref for JsonVec<T> {
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for JsonVec<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> From<Vec<T>> for JsonVec<T> {
  fn from(vec: Vec<T>) -> Self {
    Self(vec)
  }
}

impl<T> From<JsonVec<T>> for Vec<T> {
  fn from(collection: JsonVec<T>) -> Self {
    collection.0
  }
}

impl<T> FromSql for JsonVec<T>
where
  T: for<'de> Deserialize<'de>,
{
  fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
    let json = value.as_str()?;
    let collection: Vec<T> = serde_json::from_str(json).map_err(|_| FromSqlError::InvalidType)?;
    Ok(JsonVec(collection))
  }
}

impl<T> ToSql for JsonVec<T>
where
  T: Serialize,
{
  fn to_sql(&self) -> SqliteResult<ToSqlOutput<'_>> {
    let json = serde_json::to_string(&self.0)
      .map_err(|e| SqliteError::ToSqlConversionFailure(Box::new(e)))?;
    Ok(ToSqlOutput::from(json))
  }
}

#[cfg(test)]
mod test {
  use serde::{Deserialize, Serialize};

  use super::*;

  #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
  struct TestItem {
    id: u32,
    name: String,
  }

  impl TestItem {
    fn new(id: u32, name: impl Into<String>) -> Self {
      Self {
        id,
        name: name.into(),
      }
    }
  }

  mod clear {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_removes_all_items() {
      let mut json_vec = JsonVec::new();
      json_vec.push(TestItem::new(1, "test"));
      json_vec.push(TestItem::new(2, "test2"));

      assert_eq!(json_vec.len(), 2);

      json_vec.clear();

      assert!(json_vec.is_empty());
      assert_eq!(json_vec.len(), 0);
    }
  }

  mod default {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_creates_empty_json_vec() {
      let json_vec = JsonVec::<TestItem>::default();

      assert!(json_vec.is_empty());
      assert_eq!(json_vec.len(), 0);
    }
  }

  mod deref {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_allows_vec_operations() {
      let mut json_vec = JsonVec::new();
      json_vec.push(TestItem::new(1, "first"));
      json_vec.push(TestItem::new(2, "second"));

      assert_eq!(json_vec.len(), 2);
      assert_eq!(json_vec[0], TestItem::new(1, "first"));
      assert_eq!(json_vec.last(), Some(&TestItem::new(2, "second")));
    }
  }

  mod from_sql {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_deserializes_from_json_string() {
      let connection = Connection::open_in_memory().unwrap();
      let json = r#"[{"id":1,"name":"test"},{"id":2,"name":"test2"}]"#;
      let mut statement = connection.prepare("SELECT ?1").unwrap();

      let result: JsonVec<TestItem> = statement.query_row([json], |row| row.get(0)).unwrap();

      let expected = JsonVec::from(vec![TestItem::new(1, "test"), TestItem::new(2, "test2")]);

      assert_eq!(result, expected);
    }

    #[test]
    fn it_handles_empty_array() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection.prepare("SELECT '[]'").unwrap();

      let result: JsonVec<TestItem> = statement.query_row([], |row| row.get(0)).unwrap();

      assert!(result.is_empty());
    }

    #[test]
    fn it_returns_error_for_invalid_json() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection.prepare("SELECT 'invalid json'").unwrap();

      let result: Result<JsonVec<TestItem>, _> = statement.query_row([], |row| row.get(0));

      assert!(result.is_err());
    }
  }

  mod from_vec {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_from_vec() {
      let vec = vec![TestItem::new(1, "test"), TestItem::new(2, "test2")];
      let json_vec = JsonVec::from(vec);

      assert_eq!(json_vec.len(), 2);
      assert_eq!(json_vec[0], TestItem::new(1, "test"));
      assert_eq!(json_vec[1], TestItem::new(2, "test2"));
    }
  }

  mod is_empty {
    use super::*;

    #[test]
    fn it_returns_true_for_empty_json_vec() {
      let json_vec = JsonVec::<TestItem>::new();
      assert!(json_vec.is_empty());
    }

    #[test]
    fn it_returns_false_for_non_empty_json_vec() {
      let mut json_vec = JsonVec::new();
      json_vec.push(TestItem::new(1, "test"));
      assert!(!json_vec.is_empty());
    }
  }

  mod len {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_number_of_items() {
      let mut json_vec = JsonVec::new();
      assert_eq!(json_vec.len(), 0);

      json_vec.push(TestItem::new(1, "test"));
      assert_eq!(json_vec.len(), 1);

      json_vec.push(TestItem::new(2, "test2"));
      assert_eq!(json_vec.len(), 2);
    }
  }

  mod push {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_item_to_json_vec() {
      let mut json_vec = JsonVec::new();
      let item = TestItem::new(1, "test");

      json_vec.push(item.clone());

      assert_eq!(json_vec.len(), 1);
      assert_eq!(json_vec[0], item);
    }
  }

  mod retain {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_removes_items_that_do_not_match_predicate() {
      let mut json_vec = JsonVec::new();
      json_vec.push(TestItem::new(1, "test"));
      json_vec.push(TestItem::new(2, "test2"));

      json_vec.retain(|item| item.id != 1);

      assert_eq!(json_vec.len(), 1);
      assert_eq!(json_vec[0], TestItem::new(2, "test2"));
    }
  }

  mod to_sql {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_serializes_to_json_string() {
      let connection = Connection::open_in_memory().unwrap();

      connection
        .execute(
          "CREATE TABLE test_table (id INTEGER PRIMARY KEY, items TEXT)",
          [],
        )
        .unwrap();

      let mut json_vec = JsonVec::new();
      json_vec.push(TestItem::new(1, "test"));
      json_vec.push(TestItem::new(2, "test2"));

      connection
        .execute("INSERT INTO test_table (items) VALUES (?1)", [&json_vec])
        .unwrap();

      let stored_json: String = connection
        .prepare("SELECT items FROM test_table")
        .unwrap()
        .query_row([], |row| row.get(0))
        .unwrap();

      let expected_json = r#"[{"id":1,"name":"test"},{"id":2,"name":"test2"}]"#;
      assert_eq!(stored_json, expected_json);
    }

    #[test]
    fn it_serializes_empty_json_vec() {
      let connection = Connection::open_in_memory().unwrap();

      connection
        .execute(
          "CREATE TABLE test_table (id INTEGER PRIMARY KEY, items TEXT)",
          [],
        )
        .unwrap();

      let json_vec = JsonVec::<TestItem>::new();

      connection
        .execute("INSERT INTO test_table (items) VALUES (?1)", [&json_vec])
        .unwrap();

      let stored_json: String = connection
        .prepare("SELECT items FROM test_table")
        .unwrap()
        .query_row([], |row| row.get(0))
        .unwrap();

      assert_eq!(stored_json, "[]");
    }
  }

  mod to_vec {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_vec() {
      let mut json_vec = JsonVec::new();
      json_vec.push(TestItem::new(1, "test"));
      json_vec.push(TestItem::new(2, "test2"));

      let vec: Vec<TestItem> = json_vec.into();

      assert_eq!(vec.len(), 2);
      assert_eq!(vec[0], TestItem::new(1, "test"));
      assert_eq!(vec[1], TestItem::new(2, "test2"));
    }
  }
}
