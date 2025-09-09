use diesel::prelude::*;
use eyre::Result;

use super::{Project, entities::Update, types::UpdateStatus};
use crate::{models::schema::project_updates, store::with_connection};

impl Project {
  pub fn add_update(&mut self, status: UpdateStatus) -> Result<Update> {
    let update = Update::new(self.id(), status);

    with_connection(|connection| {
      diesel::insert_into(project_updates::table)
        .values(&update)
        .execute(connection)?;

      Ok(update)
    })
  }

  pub fn add_update_with_description(
    &mut self,
    status: UpdateStatus,
    description: impl Into<String>,
  ) -> Result<Update> {
    let update = Update::new(self.id(), status).with_description(description);

    with_connection(|connection| {
      diesel::insert_into(project_updates::table)
        .values(&update)
        .execute(connection)?;

      Ok(update)
    })
  }

  pub fn remove_update(&mut self, update_id: impl Into<String>) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(
        project_updates::table
          .filter(project_updates::id.eq(update_id.into()))
          .filter(project_updates::project_id.eq(self.id())),
      )
      .execute(connection)?;

      Ok(())
    })
  }
}
