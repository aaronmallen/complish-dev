use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use eyre::{Error, Result, eyre};
use getset::Getters;
use rusqlite::{
  Result as SqliteResult,
  types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Eq, Getters, PartialEq)]
pub struct RGB {
  #[get = "pub"]
  blue: u8,
  #[get = "pub"]
  green: u8,
  #[get = "pub"]
  red: u8,
}

impl RGB {
  pub fn new(red: u8, green: u8, blue: u8) -> Self {
    Self {
      blue,
      green,
      red,
    }
  }

  pub fn random() -> Self {
    Self {
      blue: fastrand::u8(0..=255),
      green: fastrand::u8(0..=255),
      red: fastrand::u8(0..=255),
    }
  }
}

impl Display for RGB {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let hex = format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue).to_uppercase();
    write!(f, "{hex}")
  }
}

impl<'de> Deserialize<'de> for RGB {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let hex = String::deserialize(deserializer)?;
    Self::from_str(&hex).map_err(serde::de::Error::custom)
  }
}

impl FromSql for RGB {
  fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
    let hex = value.as_str()?;
    Self::from_str(hex).map_err(|_| FromSqlError::InvalidType)
  }
}

impl FromStr for RGB {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let hex = s.trim_start_matches('#');

    if hex.len() != 6 && hex.len() != 3 {
      return Err(eyre!(
        "Hex color must be exactly 3 or 6 characters, got {}",
        hex.len()
      ));
    }

    let hex: &str = if hex.len() == 3 {
      let chars: Vec<char> = hex.chars().collect();
      &format!(
        "{}{}{}{}{}{}",
        chars[0], chars[0], chars[1], chars[1], chars[2], chars[2]
      )
    } else {
      hex
    };

    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
      return Err(eyre!(
        "Hex color contains invalid characters. Only 0-9, a-f, A-F are allowed"
      ));
    }

    let red =
      u8::from_str_radix(&hex[0..2], 16).map_err(|_| eyre!("Failed to parse red component"))?;
    let green =
      u8::from_str_radix(&hex[2..4], 16).map_err(|_| eyre!("Failed to parse green component"))?;
    let blue =
      u8::from_str_radix(&hex[4..6], 16).map_err(|_| eyre!("Failed to parse blue component"))?;

    Ok(Self::new(red, green, blue))
  }
}

impl Serialize for RGB {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

impl ToSql for RGB {
  fn to_sql(&self) -> SqliteResult<ToSqlOutput<'_>> {
    Ok(ToSqlOutput::from(self.to_string()))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod deserialize {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_deserializes_from_hex_string() {
      let json = "\"#FF0000\"";
      let color: RGB = serde_json::from_str(json).unwrap();
      assert_eq!(color, RGB::new(255, 0, 0));
    }

    #[test]
    fn it_deserializes_short_hex() {
      let json = "\"#F00\"";
      let color: RGB = serde_json::from_str(json).unwrap();
      assert_eq!(color, RGB::new(255, 0, 0));
    }

    #[test]
    fn it_errors_on_invalid_hex() {
      let json = "\"#GGGGGG\"";
      let result: Result<RGB, _> = serde_json::from_str(json);
      assert!(result.is_err());
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_hex_string() {
      let black = RGB::new(0, 0, 0);
      let blue = RGB::new(0, 0, 255);
      let green = RGB::new(0, 255, 0);
      let red = RGB::new(255, 0, 0);
      let white = RGB::new(255, 255, 255);

      assert_eq!(black.to_string(), "#000000");
      assert_eq!(blue.to_string(), "#0000FF");
      assert_eq!(green.to_string(), "#00FF00");
      assert_eq!(red.to_string(), "#FF0000");
      assert_eq!(white.to_string(), "#FFFFFF");
    }
  }

  mod from_sql {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_returns_an_rgb_color() {
      let connection = Connection::open_in_memory().unwrap();
      let mut statement = connection.prepare("SELECT '#000000'").unwrap();
      let result: RGB = statement.query_row([], |row| row.get(0)).unwrap();

      assert_eq!(result, RGB::new(0, 0, 0));
    }
  }

  mod from_str {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_an_rgb_color() {
      let color = RGB::from_str("#000000").unwrap();

      assert_eq!(color, RGB::new(0, 0, 0));
    }

    #[test]
    fn it_supports_short_hex_colors() {
      let color = RGB::from_str("#000").unwrap();

      assert_eq!(color, RGB::new(0, 0, 0));
    }

    #[test]
    fn it_returns_an_error_for_invalid_hex_colors() {
      assert!(RGB::from_str("#GGGGGG").is_err());
    }
  }

  mod random {
    use super::*;

    #[test]
    fn it_returns_a_random_rgb_color() {
      let color = RGB::random();

      assert!(color.red() >= &0);
      assert!(color.red() <= &255);
      assert!(color.green() >= &0);
      assert!(color.green() <= &255);
      assert!(color.blue() >= &0);
      assert!(color.blue() <= &255);
    }
  }

  mod serialize {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_serializes_to_hex_string() {
      let red = RGB::new(255, 0, 0);
      let json = serde_json::to_string(&red).unwrap();
      assert_eq!(json, "\"#FF0000\"");
    }
  }

  mod to_sql {
    use pretty_assertions::assert_eq;
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_stores_rgb_color_as_hex_string() {
      let connection = Connection::open_in_memory().unwrap();

      connection
        .execute(
          "CREATE TABLE colors (id INTEGER PRIMARY KEY, color TEXT)",
          [],
        )
        .unwrap();

      let black = RGB::new(0, 0, 0);
      let blue = RGB::new(0, 0, 255);
      let red = RGB::new(255, 0, 0);

      connection
        .execute(
          "INSERT INTO colors (color) VALUES (?1), (?2), (?3)",
          [&black, &blue, &red],
        )
        .unwrap();

      let stored_colors: Vec<String> = connection
        .prepare("SELECT color FROM colors ORDER BY id")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

      assert_eq!(stored_colors, vec!["#000000", "#0000FF", "#FF0000"]);
    }
  }
}
