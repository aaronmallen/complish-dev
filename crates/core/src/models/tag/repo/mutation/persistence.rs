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
