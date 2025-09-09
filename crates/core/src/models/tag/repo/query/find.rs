use diesel::prelude::*;
use eyre::Result;

use super::Tag;
use crate::{models::schema::tags, store::with_connection};

impl Tag {
  pub fn find(id: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        tags::table
          .find(id.into())
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }

  pub fn find_by_label(label: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        tags::table
          .filter(tags::label.eq(label.into()))
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::macros::with_test_connection;

  mod find {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_finds_the_tag_if_it_exists() {
      with_test_connection!({
        let mut tag = Tag::new("test");
        tag.save().unwrap();

        assert_eq!(tag, Tag::find(tag.id()).unwrap());
      })
    }

    #[test]
    fn it_returns_an_error_if_the_tag_does_not_exist() {
      with_test_connection!({
        assert!(Tag::find("ilh60qwnip9p03i70cx3vzvi").is_err());
      })
    }
  }

  mod find_by_label {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_finds_the_tag_if_it_exists() {
      with_test_connection!({
        let mut tag = Tag::new("test");
        tag.save().unwrap();

        assert_eq!(tag, Tag::find_by_label("test").unwrap());
      })
    }

    #[test]
    fn it_returns_an_error_if_the_tag_does_not_exist() {
      with_test_connection!({
        assert!(Tag::find_by_label("test").is_err());
      })
    }
  }
}
