use chrono::{NaiveDateTime, Utc};
use color_eyre::Result;
use diesel::{Identifiable, Insertable, Queryable, Selectable, prelude::*};
use getset::{Getters, Setters};

use super::schema::accomplishments;
use crate::store::with_connection;

#[derive(
  Clone, Debug, Eq, Getters, Identifiable, Insertable, PartialEq, Queryable, Selectable, Setters,
)]
#[diesel(table_name = accomplishments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Accomplishment {
  #[get = "pub"]
  id: String,
  #[getset(get = "pub", set = "pub")]
  content: String,
  #[get = "pub"]
  created_at: NaiveDateTime,
  #[getset(get = "pub", set = "pub")]
  updated_at: NaiveDateTime,
}

impl Accomplishment {
  pub fn new(content: impl Into<String>) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      id: cuid2::create_id(),
      content: content.into(),
      created_at: now,
      updated_at: now,
    }
  }

  pub fn find(id: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        accomplishments::table
          .find(id.into())
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }

  pub fn find_for_date_range(start: NaiveDateTime, end: NaiveDateTime) -> Result<Vec<Self>> {
    with_connection(|connection| {
      Ok(
        accomplishments::table
          .filter(accomplishments::created_at.between(start, end))
          .select(Self::as_select())
          .load(connection)?,
      )
    })
  }

  pub fn save(&self) -> Result<Self> {
    with_connection(|connection| {
      diesel::insert_into(accomplishments::table)
        .values(self)
        .on_conflict(accomplishments::id)
        .do_update()
        .set((
          accomplishments::content.eq(&self.content),
          accomplishments::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(connection)?;

      Ok(())
    })?;

    Self::find(self.id())
  }
}
