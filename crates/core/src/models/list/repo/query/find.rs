use diesel::prelude::*;
use eyre::Result;

use super::List;
use crate::{models::schema::lists, store::with_connection};

impl List {
  pub fn find(id: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        lists::table
          .find(id.into())
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }

  pub fn find_by_name(name: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        lists::table
          .filter(lists::name.eq(name.into()))
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }
}
