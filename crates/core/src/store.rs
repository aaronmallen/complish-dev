use std::{fs, path::PathBuf, sync::OnceLock};

use diesel::{
  Connection, SqliteConnection,
  r2d2::{ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use eyre::{Result, eyre};

use crate::env::{self, Value as EnvValue};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

static POOL: OnceLock<DbPool> = OnceLock::new();

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
    .map_err(|e| eyre!("Failed to run pending migrations: {}", e))?;

  POOL
    .set(pool)
    .map_err(|_| eyre!("Failed to initialize database pool"))?;

  Ok(())
}

pub fn with_connection<F, R>(f: F) -> eyre::Result<R>
where
  F: FnOnce(&mut SqliteConnection) -> eyre::Result<R>,
{
  let pool = get_pool();
  let mut connection = pool.get()?;
  f(&mut connection)
}

pub fn with_transaction<F, R>(f: F) -> eyre::Result<R>
where
  F: FnOnce(&mut SqliteConnection) -> eyre::Result<R>,
{
  let pool = get_pool();
  let mut connection = pool.get()?;
  connection.transaction(|conn| f(conn))
}

fn database_url() -> Result<String> {
  let data_home = data_dir()?;

  fs::create_dir_all(&data_home).map_err(|e| {
    eyre!(
      "Could not create data directory: {} - {}",
      data_home.display(),
      e
    )
  })?;

  Ok(format!("sqlite://{}", data_home.join("store").display()))
}

fn data_dir() -> Result<PathBuf> {
  match &*env::COMPLISH_DATA_DIR {
    EnvValue::Ok(path) => Ok(path.to_owned()),
    _ => dir_spec::data_home()
      .map(|p| p.join("complish"))
      .ok_or_else(|| {
        eyre!(
          "Could not find user's data directory. Please set COMPLISH_DATA_DIR in your environment"
        )
      }),
  }
}

fn get_pool() -> &'static DbPool {
  POOL
    .get()
    .expect("Database pool not initialized. Call `connect` first.")
}

#[cfg(test)]
mod tests {
  use super::*;

  mod connect {
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_works() {
      let tmp_dir = tempfile::tempdir().unwrap();
      let data_dir = tmp_dir.path().join("data");

      with_var(
        "COMPLISH_DATA_DIR",
        Some(data_dir.to_str().unwrap()),
        || {
          assert!(connect().is_ok());
        },
      );
    }
  }

  mod with_connection {
    use diesel::prelude::*;
    use pretty_assertions::assert_eq;
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_works() {
      let tmp_dir = tempfile::tempdir().unwrap();
      let data_dir = tmp_dir.path().join("data");

      with_var(
        "COMPLISH_DATA_DIR",
        Some(data_dir.to_str().unwrap()),
        || {
          connect().unwrap();

          let result = with_connection(|connection| {
            diesel::sql_query("SELECT 1").execute(connection)?;
            Ok(42)
          });

          assert!(result.is_ok());
          assert_eq!(result.unwrap(), 42);
        },
      );
    }
  }

  mod with_transactions {
    use diesel::prelude::*;
    use pretty_assertions::assert_eq;
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_works() {
      let tmp_dir = tempfile::tempdir().unwrap();
      let data_dir = tmp_dir.path().join("data");

      with_var(
        "COMPLISH_DATA_DIR",
        Some(data_dir.to_str().unwrap()),
        || {
          connect().unwrap();

          let result = with_transaction(|connection| {
            diesel::sql_query("SELECT 1").execute(connection)?;
            Ok("success")
          });

          assert!(result.is_ok());
          assert_eq!(result.unwrap(), "success");
        },
      );
    }
  }
}
