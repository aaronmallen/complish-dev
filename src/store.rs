use std::{fs, sync::OnceLock};

use color_eyre::{Result, eyre::eyre};
use diesel::{
  SqliteConnection,
  connection::SimpleConnection,
  r2d2::{ConnectionManager, Pool},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

static POOL: OnceLock<Pool<ConnectionManager<SqliteConnection>>> = OnceLock::new();

pub fn connect() -> Result<()> {
  if POOL.get().is_some() {
    return Ok(());
  }

  let url = database_url()?;
  let manager = ConnectionManager::<SqliteConnection>::new(url);
  let pool = Pool::builder().max_size(10).build(manager)?;

  pool
    .get()?
    .run_pending_migrations(MIGRATIONS)
    .map_err(|e| eyre!("Failed to run migrations: {}", e))?;

  POOL
    .set(pool)
    .map_err(|_| eyre!("Failed to initialize database pool"))?;

  Ok(())
}

pub fn with_connection<F, R>(f: F) -> Result<R>
where
  F: FnOnce(&mut SqliteConnection) -> Result<R>,
{
  let pool = get_pool();
  let mut connection = pool.get()?;
  connection.batch_execute("PRAGMA foreign_keys = ON")?;
  f(&mut connection)
}

fn database_url() -> Result<String> {
  let store_file = dir_spec::data_home()
    .map(|p| p.join("complish/store.db"))
    .ok_or_else(|| eyre!("Could not determine user's data directory"))?;

  fs::create_dir_all(
    store_file
      .parent()
      .ok_or_else(|| eyre!("Could not determine parent directory"))?,
  )
  .map_err(|e| eyre!("Could not create store: {}", e))?;

  Ok(format!("sqlite://{}", store_file.display()))
}

fn get_pool() -> &'static Pool<ConnectionManager<SqliteConnection>> {
  POOL
    .get()
    .expect("Database pool not initialized. Call `connect` first.")
}
