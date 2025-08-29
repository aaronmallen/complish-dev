use std::str::FromStr;

use chrono::{DateTime, Utc};
use cuid::cuid2;
use eyre::Result;
use getset::Getters;
use rusqlite::{Error as SqliteError, Result as SqliteResult, Row, types::ToSql};
use serde::{Deserialize, Serialize};

use crate::entities::RGB;

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct Tag {
  #[get = "pub"]
  color: RGB,
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get = "pub"]
  id: String,
  #[get = "pub"]
  label: String,
  #[get = "pub"]
  updated_at: DateTime<Utc>,
}

impl Tag {
  pub fn new(label: impl Into<String>) -> Self {
    let now = Utc::now();

    Self {
      color: RGB::random(),
      created_at: now,
      id: cuid2(),
      label: label.into(),
      updated_at: now,
    }
  }

  pub fn to_sql_params(&self) -> [&dyn ToSql; 5] {
    [
      &self.id,
      &self.label,
      &self.color,
      &self.created_at,
      &self.updated_at,
    ]
  }

  pub fn touch(&mut self) {
    self.updated_at = Utc::now();
  }

  #[must_use = "This method returns a new Tag with the color set"]
  pub fn with_color(mut self, red: u8, green: u8, blue: u8) -> Self {
    self.color = RGB::new(red, green, blue);
    self
  }

  pub fn with_hex_color(mut self, hex: impl Into<String>) -> Result<Self> {
    self.color = RGB::from_str(&hex.into())?;
    Ok(self)
  }

  pub fn update_color(&mut self, red: u8, green: u8, blue: u8) {
    self.color = RGB::new(red, green, blue);
    self.touch();
  }

  pub fn update_hex_color(&mut self, hex: impl Into<String>) -> Result<()> {
    self.color = RGB::from_str(&hex.into())?;
    self.touch();
    Ok(())
  }
}

impl TryFrom<&Row<'_>> for Tag {
  type Error = SqliteError;

  fn try_from(row: &Row<'_>) -> SqliteResult<Self> {
    Ok(Self {
      color: row.get("color")?,
      created_at: row.get("created_at")?,
      id: row.get("id")?,
      label: row.get("label")?,
      updated_at: row.get("updated_at")?,
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod to_sql_params {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_returns_the_tag_fields_as_sql_params() {
      let connection = Connection::open_in_memory().unwrap();
      connection
        .execute(
          r"
        CREATE TABLE tags (
          id TEXT PRIMARY KEY,
          label TEXT NOT NULL,
          color TEXT NOT NULL,
          created_at DATETIME NOT NULL,
          updated_at DATETIME NOT NULL
        );
      ",
          [],
        )
        .unwrap();

      let tag = Tag::new("test");

      let result = connection.execute(
        r"
          INSERT INTO tags (id, label, color, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)
        ",
        tag.to_sql_params(),
      );

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), 1);
    }
  }

  mod touch {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_updates_the_updated_at_field() {
      let mut tag = Tag::new("test");
      let old_updated_at = Utc::now() - chrono::Duration::seconds(10);
      tag.updated_at = old_updated_at;

      assert_eq!(&old_updated_at, tag.updated_at());

      tag.touch();

      assert_ne!(&old_updated_at, tag.updated_at());
      assert!(tag.updated_at() > &old_updated_at);
    }
  }

  mod try_from_row {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_returns_a_tag() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection
        .prepare(
          r"
          SELECT
            'popmfy0xiz8ykp3urgojxtma' AS id,
            'test' AS label,
            '#000000' AS color,
            datetime('now', 'utc') AS created_at,
            datetime('now', 'utc') AS updated_at
        ",
        )
        .unwrap();
      let tag = statement.query_row([], |row| Tag::try_from(row)).unwrap();

      assert_eq!(tag.id(), "popmfy0xiz8ykp3urgojxtma");
      assert_eq!(tag.label(), "test");
      assert_eq!(tag.color(), &RGB::new(0, 0, 0));
    }
  }

  mod with_color {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_tag_with_the_color_set() {
      let tag = Tag::new("test").with_color(1, 2, 3);

      assert_eq!(tag.color(), &RGB::new(1, 2, 3));
    }
  }

  mod with_hex_color {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_tag_with_the_color_set() {
      let tag = Tag::new("test").with_hex_color("#000000").unwrap();

      assert_eq!(tag.color(), &RGB::new(0, 0, 0));
    }
  }

  mod update_color {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_color_field() {
      let mut tag = Tag::new("test").with_color(0, 0, 0);

      assert_eq!(tag.color(), &RGB::new(0, 0, 0));

      tag.update_color(255, 255, 255);

      assert_eq!(tag.color(), &RGB::new(255, 255, 255));
    }
  }

  mod update_hex_color {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_color_field() {
      let mut tag = Tag::new("test").with_color(0, 0, 0);

      assert_eq!(tag.color(), &RGB::new(0, 0, 0));

      tag.update_hex_color("#ffffff").unwrap();

      assert_eq!(tag.color(), &RGB::new(255, 255, 255));
    }
  }
}
