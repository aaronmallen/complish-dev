use diesel::prelude::*;
use eyre::Result;

use super::Tag;
use crate::{models::schema::tags, store::with_connection};

impl Tag {
  pub fn all() -> Result<Vec<Self>> {
    with_connection(|connection| Ok(tags::table.select(Self::as_select()).load(connection)?))
  }
}
