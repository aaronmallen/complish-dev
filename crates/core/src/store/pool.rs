use std::sync::OnceLock;

use diesel::{
  Connection, SqliteConnection,
  connection::SimpleConnection,
  r2d2::{ConnectionManager, Pool},
};
use eyre::{Result, eyre};

use super::{migration, path};

static POOL: OnceLock<Pool<ConnectionManager<SqliteConnection>>> = OnceLock::new();

pub fn connect() -> Result<()> {
  if POOL.get().is_some() {
    return Ok(());
  }

  let url = path::database_url()?;
  let manager = ConnectionManager::<SqliteConnection>::new(url);
  let pool = Pool::builder().max_size(10).build(manager)?;

  migration::run(&mut *pool.get()?)?;

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

pub fn with_transaction<F, R>(f: F) -> Result<R>
where
  F: FnOnce(&mut SqliteConnection) -> Result<R>,
{
  let pool = get_pool();
  let mut connection = pool.get()?;
  connection.batch_execute("PRAGMA foreign_keys = ON")?;
  connection.transaction(|conn| f(conn))
}

fn get_pool() -> &'static Pool<ConnectionManager<SqliteConnection>> {
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
