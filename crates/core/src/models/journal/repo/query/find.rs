use diesel::prelude::*;
use eyre::Result;

use super::Entry;
use crate::{models::schema::journal_entries, store::with_connection};

impl Entry {
  pub fn find(id: impl Into<String>) -> Result<Self> {
    with_connection(|connection| {
      Ok(
        journal_entries::table
          .find(id.into())
          .select(Self::as_select())
          .first(connection)?,
      )
    })
  }
}
