use eyre::Result;
use rusqlite::Connection;

use crate::{
  config::Config, journal::Repo as JournalRepo, project::Repo as ProjectRepo, tag::Repo as TagRepo,
  task::Repo as TaskRepo,
};

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

  pub fn journal(&self) -> JournalRepo<'_> {
    JournalRepo::new(&self.connection)
  }

  pub fn project(&self) -> ProjectRepo<'_> {
    ProjectRepo::new(&self.connection)
  }

  pub fn tag(&self) -> TagRepo<'_> {
    TagRepo::new(&self.connection)
  }

  pub fn task(&self) -> TaskRepo<'_> {
    TaskRepo::new(&self.connection)
  }
}
