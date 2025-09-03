use eyre::Result;
use rusqlite::Connection;

use crate::{config::Config, tag::Repo as TagRepo};

pub struct Repo {
  connection: Connection,
}

impl Repo {
  pub fn new(config: Config) -> Result<Self> {
    let store_file = config.settings().core().store_file();
    let connection = Connection::open(store_file)?;

    Ok(Self {
      connection,
    })
  }

  pub fn tag(&self) -> TagRepo<'_> {
    TagRepo::new(&self.connection)
  }
}
