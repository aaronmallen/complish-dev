use chrono::{DateTime, Utc};
use cuid::cuid2;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct Note {
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get = "pub"]
  content: String,
  #[get = "pub"]
  id: String,
  #[get = "pub"]
  date: DateTime<Utc>,
  #[get = "pub"]
  updated_at: DateTime<Utc>,
}

impl Note {
  pub fn new(content: impl Into<String>) -> Self {
    let now = Utc::now();
    Self {
      created_at: now,
      content: content.into(),
      id: cuid2(),
      date: Utc::now(),
      updated_at: now,
    }
  }

  pub fn update(&mut self, content: impl Into<String>) {
    self.content = content.into();
    self.updated_at = Utc::now();
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod update {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_updates_the_content() {
      let mut note = Note::new("old content");
      note.update("new content");

      assert_eq!(note.content(), &"new content");
    }
  }
}
