use chrono::Utc;
use diesel::prelude::*;
use eyre::Result;

use super::Tag;
use crate::{models::schema::tags, store::with_connection};

impl Tag {
  pub fn find_or_create(label: impl Into<String>) -> Result<Self> {
    let label = label.into();

    with_connection(|connection| match Self::find_by_label(&label) {
      Ok(tag) => Ok(tag),
      Err(_) => {
        let tag = Self::new(&label);
        diesel::insert_into(tags::table)
          .values(&tag)
          .execute(connection)?;
        Ok(tag)
      }
    })
  }

  pub fn delete(&self) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(tags::table.find(self.id())).execute(connection)?;
      Ok(())
    })
  }

  pub fn save(&mut self) -> Result<()> {
    with_connection(|connection| {
      let updated = diesel::update(tags::table.find(self.id()))
        .set((
          tags::label.eq(self.label()),
          tags::metadata.eq(self.metadata()),
          tags::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(connection)?;

      if updated > 0 {
        self.set_updated_at(Utc::now().naive_utc());
      } else {
        diesel::insert_into(tags::table)
          .values(&*self)
          .on_conflict(tags::label)
          .do_nothing()
          .execute(connection)?;
      }

      Ok(())
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::macros::with_test_connection;

  mod find_or_create {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_finds_the_tag_if_it_exists() {
      with_test_connection!({
        let mut tag = Tag::new("test");
        tag.save().unwrap();

        assert_eq!(tag, Tag::find_or_create("test").unwrap());
      })
    }

    #[test]
    fn it_creates_the_tag_if_it_does_not_exist() {
      with_test_connection!({
        let tag = Tag::find_or_create("test").unwrap();
        assert_eq!("test", tag.label());
      })
    }
  }

  mod delete {
    use super::*;

    #[test]
    fn it_deletes_the_tag() {
      with_test_connection!({
        let mut tag = Tag::new("test");
        tag.save().unwrap();
        tag.delete().unwrap();
        assert!(Tag::find(tag.id()).is_err());
      })
    }
  }

  mod save {
    use pretty_assertions::assert_ne;

    use super::*;

    #[test]
    fn it_saves_the_tag() {
      with_test_connection!({
        let mut tag = Tag::new("test");
        tag.save().unwrap();

        assert_eq!(tag, Tag::find(tag.id()).unwrap());
      })
    }

    #[test]
    fn it_updates_the_tag_if_it_already_exists() {
      with_test_connection!({
        let mut tag = Tag::new("test");
        tag.save().unwrap();
        tag.save().unwrap();

        assert_ne!(tag.updated_at(), Tag::find(tag.id()).unwrap().updated_at());
      })
    }
  }
}
