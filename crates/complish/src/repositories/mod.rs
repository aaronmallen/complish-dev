mod tag_repository;

use std::path::PathBuf;

use eyre::Result;
use rusqlite::Connection;
use tag_repository::TagRepository;

pub struct Repository {
  connection: Connection,
}

impl Repository {
  pub fn new(store_file: PathBuf) -> Result<Self> {
    let connection = Connection::open(store_file)?;

    Ok(Self {
      connection,
    })
  }

  pub fn tag(&'_ self) -> TagRepository<'_> {
    TagRepository::new(&self.connection)
  }
}
