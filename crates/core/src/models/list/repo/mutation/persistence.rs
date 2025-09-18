use diesel::prelude::*;
use eyre::Result;

use super::List;
use crate::{models::schema::lists, store::with_connection};

impl List {
  pub fn find_or_create(name: impl Into<String>) -> Result<Self> {
    let name = name.into();

    with_connection(|connection| match Self::find_by_name(&name) {
      Ok(list) => Ok(list),
      Err(_) => {
        let list = Self::new(&name);
        diesel::insert_into(lists::table)
          .values(&list)
          .execute(connection)?;
        Ok(list)
      }
    })
  }

  pub fn delete(&self) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(lists::table.find(self.id())).execute(connection)?;

      Ok(())
    })
  }

  pub fn save(&mut self) -> Result<()> {
    with_connection(|connection| {
      diesel::insert_into(lists::table)
        .values(&*self)
        .on_conflict(lists::id)
        .do_update()
        .set((
          lists::directories.eq(self.directories()),
          lists::metadata.eq(self.metadata()),
          lists::name.eq(self.name()),
          lists::updated_at.eq(self.updated_at()),
        ))
        .execute(connection)?;

      Ok(())
    })
  }
}
