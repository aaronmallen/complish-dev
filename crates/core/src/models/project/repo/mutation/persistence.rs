use chrono::Utc;
use diesel::prelude::*;
use eyre::Result;

use super::Project;
use crate::{models::schema::projects, store::with_connection};

impl Project {
  pub fn create(name: impl Into<String>) -> Result<Self> {
    let project = Project::new(name);

    with_connection(|connection| {
      diesel::insert_into(projects::table)
        .values(&project)
        .execute(connection)?;

      Ok(())
    })?;

    Self::find(project.id())
  }

  pub fn delete(&self) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(projects::table.filter(projects::id.eq(self.id()))).execute(connection)?;

      Ok(())
    })
  }

  pub fn save(&mut self) -> Result<()> {
    with_connection(|connection| {
      diesel::insert_into(projects::table)
        .values(&*self)
        .on_conflict(projects::id)
        .do_update()
        .set((
          projects::completed_at.eq(self.completed_at()),
          projects::directories.eq(self.directories()),
          projects::description.eq(self.description()),
          projects::metadata.eq(self.metadata()),
          projects::resolution.eq(self.resolution()),
          projects::updated_at.eq(Utc::now().naive_utc()),
          projects::workflow_status.eq(self.workflow_status()),
        ))
        .execute(connection)?;

      Ok(())
    })
  }
}
