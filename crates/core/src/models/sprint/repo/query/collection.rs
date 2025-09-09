use diesel::prelude::*;
use eyre::Result;

use super::Sprint;
use crate::{
  models::schema::sprints,
  store::with_connection,
};

impl Sprint {
  pub fn all() -> Result<Vec<Self>> {
    with_connection(|connection| Ok(sprints::table.select(Self::as_select()).load(connection)?))
  }
}
