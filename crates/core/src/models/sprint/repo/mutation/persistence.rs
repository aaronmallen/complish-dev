use chrono::Utc;
use diesel::prelude::*;
use eyre::{Result, eyre};

use super::Sprint;
use crate::{
  models::schema::sprints,
  store::with_connection,
};

impl Sprint {
  pub fn create() -> Result<Self> {
    if let Ok(current) = Self::current() {
      return Ok(current);
    }

    let sprint = Sprint::new();

    with_connection(|connection| {
      diesel::insert_into(sprints::table)
        .values(&sprint)
        .execute(connection)?;

      Ok(())
    })?;

    Self::find(sprint.id())
  }

  pub fn close() -> Result<()> {
    match Self::current() {
      Ok(mut current) => {
        current.set_ended_at(Utc::now().naive_utc());
        current.save()
      }
      _ => Err(eyre!("No sprint is currently running.")),
    }
  }

  pub fn delete(&self) -> Result<()> {
    with_connection(|connection| {
      diesel::delete(sprints::table.find(self.id())).execute(connection)?;
      Ok(())
    })
  }

  pub fn save(&mut self) -> Result<()> {
    with_connection(|connection| {
      diesel::insert_into(sprints::table)
        .values(&*self)
        .on_conflict(sprints::id)
        .do_update()
        .set((
          sprints::ended_at.eq(self.ended_at()),
          sprints::metadata.eq(self.metadata()),
          sprints::started_at.eq(self.started_at()),
          sprints::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(connection)?;

      Ok(())
    })
  }
}
