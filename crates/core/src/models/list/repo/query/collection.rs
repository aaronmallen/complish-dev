use diesel::prelude::*;
use eyre::Result;

use super::List;
use crate::{models::schema::lists, store::with_connection};

impl List {
  pub fn all() -> Result<Vec<Self>> {
    with_connection(|connection| Ok(lists::table.select(Self::as_select()).load(connection)?))
  }
}
