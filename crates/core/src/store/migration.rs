use diesel::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use eyre::{Result, eyre};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run(connection: &mut SqliteConnection) -> Result<()> {
  connection
    .run_pending_migrations(MIGRATIONS)
    .map_err(|e| eyre!("Failed to run pending migrations: {}", e))?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  mod run {
    use diesel::{Connection, RunQueryDsl};
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_works() {
      let tmp_dir = tempfile::tempdir().unwrap();
      let data_dir = tmp_dir.path().join("data");
      let db_path = data_dir.join("test.db");

      with_var(
        "COMPLISH_DATA_DIR",
        Some(data_dir.to_str().unwrap()),
        || {
          std::fs::create_dir_all(&data_dir).unwrap();

          let mut connection =
            SqliteConnection::establish(&format!("sqlite://{}", db_path.display()))
              .expect("Failed to establish connection");

          let result = run(&mut connection);
          assert!(result.is_ok());

          let result = diesel::sql_query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='__diesel_schema_migrations'"
          )
          .execute(&mut connection);

          assert!(result.is_ok(), "Migrations table should exist");
        },
      );
    }

    #[test]
    fn it_is_idempotent() {
      let tmp_dir = tempfile::tempdir().unwrap();
      let data_dir = tmp_dir.path().join("data");
      let db_path = data_dir.join("test2.db");

      with_var(
        "COMPLISH_DATA_DIR",
        Some(data_dir.to_str().unwrap()),
        || {
          std::fs::create_dir_all(&data_dir).unwrap();

          let mut connection =
            SqliteConnection::establish(&format!("sqlite://{}", db_path.display()))
              .expect("Failed to establish connection");

          let result1 = run(&mut connection);
          assert!(result1.is_ok());

          let result2 = run(&mut connection);
          assert!(result2.is_ok(), "Running migrations twice should not fail");
        },
      );
    }
  }
}
