use std::{
  collections::HashSet,
  fs,
  path::{Path, PathBuf},
  result::Result as StdResult,
};

use eyre::{Context, Result, eyre};
use log::debug;
use rusqlite::Connection;

const MIGRATIONS: &[(&str, &str)] = &[
  (
    "0001_enable_foreign_keys",
    include_str!("./migrations/0001_enable_foreign_keys.sql"),
  ),
  (
    "0002_create_tags",
    include_str!("./migrations/0002_create_tags.sql"),
  ),
  (
    "0003_create_projects",
    include_str!("./migrations/0003_create_projects.sql"),
  ),
  (
    "0004_create_project_tags",
    include_str!("./migrations/0004_create_project_tags.sql"),
  ),
];

pub fn run(store_path: &PathBuf) -> Result<()> {
  debug!("Running migrations on {}", store_path.display());

  create_store_directory(store_path)?;

  let connection = Connection::open(store_path)?;
  create_migrations_table(&connection)?;
  run_migrations(&connection)?;

  Ok(())
}

fn create_migrations_table(connection: &Connection) -> Result<()> {
  connection.execute(
    "CREATE TABLE IF NOT EXISTS schema_migrations (filename TEXT PRIMARY KEY)",
    [],
  )?;

  Ok(())
}

fn create_store_directory(store_path: &Path) -> Result<()> {
  if let Some(parent) = store_path.parent() {
    fs::create_dir_all(parent).with_context(|| {
      eyre!(
        "Failed to create data store directory: {}",
        parent.display()
      )
    })?;
  }

  Ok(())
}

fn execute_migration(connection: &Connection, sql: &str) -> Result<()> {
  connection.execute_batch(sql)?;
  Ok(())
}

fn get_pending_migrations(connection: &Connection) -> Result<Vec<&(&str, &str)>> {
  let mut statement = connection.prepare("SELECT filename FROM schema_migrations")?;
  let applied_migrations: HashSet<String> = statement
    .query_map([], |row| row.get(0))?
    .filter_map(StdResult::ok)
    .collect();

  let pending_migrations = MIGRATIONS
    .iter()
    .filter(|(filename, _)| !applied_migrations.contains(*filename))
    .collect();

  Ok(pending_migrations)
}

fn run_migrations(connection: &Connection) -> Result<()> {
  let pending_migrations = get_pending_migrations(connection)?;

  if pending_migrations.is_empty() {
    debug!("All migrations up to date");
    return Ok(());
  }

  debug!("Found {} pending migration(s)", pending_migrations.len());

  for (filename, sql) in pending_migrations {
    debug!("Running migration: {filename}");
    execute_migration(connection, sql)?;
    update_schema_version(connection, filename)?;
    debug!("Migration complete: {filename}");
  }

  Ok(())
}

fn update_schema_version(connection: &Connection, filename: &str) -> Result<()> {
  connection.execute(
    "INSERT INTO schema_migrations (filename) VALUES (?1)",
    [filename],
  )?;

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  mod run {
    use pretty_assertions::assert_eq;
    use tempdir::TempDir;

    use super::*;

    #[test]
    fn it_creates_the_store_file() {
      let temp_dir = TempDir::new("complish").unwrap();
      let store_path = temp_dir.path().join("store");
      run(&store_path).unwrap();

      assert!(store_path.exists());
    }

    #[test]
    fn it_creates_the_schema_migrations_table_and_runs_migrations() {
      let temp_dir = TempDir::new("complish").unwrap();
      let store_path = temp_dir.path().join("store");
      run(&store_path).unwrap();

      let connection = Connection::open(&store_path).unwrap();
      let mut statement = connection
        .prepare("SELECT COUNT(*) FROM schema_migrations")
        .unwrap();
      let count: u32 = statement.query_row([], |row| row.get(0)).unwrap();

      assert_eq!(count, u32::try_from(MIGRATIONS.len()).unwrap());
    }

    #[test]
    fn it_is_idempotent() {
      let temp_dir = TempDir::new("complish").unwrap();
      let store_path = temp_dir.path().join("store");
      run(&store_path).unwrap();
      run(&store_path).unwrap();

      let connection = Connection::open(&store_path).unwrap();
      let mut statement = connection
        .prepare("SELECT COUNT(*) FROM schema_migrations")
        .unwrap();
      let count: u32 = statement.query_row([], |row| row.get(0)).unwrap();

      assert_eq!(count, u32::try_from(MIGRATIONS.len()).unwrap());
    }
  }
}
