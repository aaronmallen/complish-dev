use chrono::Utc;
use diesel::prelude::*;
use eyre::Result;

use super::Entry;
use crate::{models::schema::journal_entries, store::with_connection};

impl Entry {
  pub fn create(content: impl Into<String>) -> Result<Self> {
    let entry = Entry::new(content);

    with_connection(|connection| {
      diesel::insert_into(journal_entries::table)
        .values(&entry)
        .execute(connection)?;

      Ok(())
    })?;

    Self::find(entry.id())
  }

  pub fn delete(&mut self) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(journal_entries::table.find(self.id())).execute(connection)?;
      Ok(())
    })
  }

  pub fn save(&mut self) -> Result<()> {
    with_connection(|connection| {
      diesel::insert_into(journal_entries::table)
        .values(&*self)
        .on_conflict(journal_entries::id)
        .do_update()
        .set((
          journal_entries::content.eq(self.content()),
          journal_entries::metadata.eq(self.metadata()),
          journal_entries::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(connection)?;

      Ok(())
    })
  }
}
