use diesel::prelude::*;
use eyre::Result;

use super::Entry;
use crate::{models::schema::journal_entries, store::with_connection};

impl Entry {
  pub fn all() -> Result<Vec<Self>> {
    with_connection(|connection| {
      Ok(
        journal_entries::table
          .select(Self::as_select())
          .load(connection)?,
      )
    })
  }
}
