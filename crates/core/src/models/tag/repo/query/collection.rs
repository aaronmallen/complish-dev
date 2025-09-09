use diesel::prelude::*;
use eyre::Result;

use super::Tag;
use crate::{models::schema::tags, store::with_connection};

impl Tag {
  pub fn all() -> Result<Vec<Self>> {
    with_connection(|connection| Ok(tags::table.select(Self::as_select()).load(connection)?))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod all {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::macros::with_test_connection;

    #[test]
    fn it_returns_all_tags() {
      with_test_connection!({
        let mut tag1 = Tag::new("tag1");
        let mut tag2 = Tag::new("tag2");

        tag1.save().unwrap();
        tag2.save().unwrap();

        assert_eq!(Tag::all().unwrap(), vec![tag1, tag2]);
      })
    }
  }
}
