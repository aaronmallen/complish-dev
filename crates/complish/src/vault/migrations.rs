use std::{collections::HashSet, fs, path::PathBuf, result::Result as StdResult};

use eyre::{Context, Result};
use log::debug;
use rusqlite::Connection;

const MIGRATIONS: &[(&str, &str)] = &[
  (
    "001_enable_foreign_keys.sql",
    include_str!("./migrations/001_enable_foreign_keys.sql"),
  ),
  (
    "002_create_lists.sql",
    include_str!("./migrations/002_create_lists.sql"),
  ),
  (
    "003_create_tasks.sql",
    include_str!("./migrations/003_create_tasks.sql"),
  ),
];

pub fn run(vault_path: &PathBuf) -> Result<()> {
  if let Some(parent) = vault_path.parent() {
    fs::create_dir_all(parent)
      .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
  }

  debug!("Running migrations on {}", vault_path.display());
  let connection = Connection::open(vault_path)?;

  create_migrations_table(&connection)?;
  run_migrations(&connection)?;

  debug!("All migrations completed successfully");
  Ok(())
}

fn create_migrations_table(connection: &Connection) -> Result<()> {
  connection.execute(
    "CREATE TABLE IF NOT EXISTS schema_migrations (filename TEXT PRIMARY KEY)",
    [],
  )?;

  Ok(())
}

fn run_migrations(connection: &Connection) -> Result<()> {
  let mut statement = connection.prepare("SELECT filename FROM schema_migrations")?;
  let applied_migrations: HashSet<String> = statement
    .query_map([], |row| row.get(0))?
    .filter_map(StdResult::ok)
    .collect();

  let pending_migrations: Vec<_> = MIGRATIONS
    .iter()
    .filter(|(filename, _)| !applied_migrations.contains(*filename))
    .collect();

  if pending_migrations.is_empty() {
    debug!("All migrations up to date");
    return Ok(());
  }

  debug!("Found {} pending migration(s)", pending_migrations.len());

  for (filename, sql) in pending_migrations {
    debug!("Applying migration {filename}");

    for statement in sql.split(';') {
      let statement = statement.trim();
      if !statement.is_empty() {
        connection.execute(statement, [])?;
      }
    }

    connection.execute(
      "INSERT INTO schema_migrations (filename) VALUES (?1)",
      [filename],
    )?;

    debug!("Applied migration {filename}");
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  mod run {
    use pretty_assertions::assert_eq;
    use temp_dir::TempDir;

    use super::*;

    #[test]
    fn it_creates_the_vault_directory_if_needed() {
      let temp_dir = TempDir::new().unwrap();
      let vault_path = temp_dir.path().join("complish/vault");

      assert!(!vault_path.parent().unwrap().exists());
      assert!(!vault_path.exists());

      run(&vault_path).unwrap();

      assert!(vault_path.parent().unwrap().exists());
      assert!(vault_path.exists());
    }

    #[test]
    fn it_creates_the_migrations_table_and_applies_all_migrations() {
      let temp_dir = TempDir::new().unwrap();
      let vault_path = temp_dir.path().join("complish/vault");

      run(&vault_path).unwrap();

      let connection = Connection::open(&vault_path).unwrap();
      let mut statement = connection
        .prepare("SELECT COUNT(*) FROM schema_migrations")
        .unwrap();
      let count: u32 = statement.query_row([], |row| row.get(0)).unwrap();

      assert_eq!(count, u32::try_from(MIGRATIONS.len()).unwrap());
    }

    #[test]
    fn it_is_idempotent() {
      let temp_dir = TempDir::new().unwrap();
      let vault_path = temp_dir.path().join("complish/vault");

      run(&vault_path).unwrap();
      run(&vault_path).unwrap();

      let connection = Connection::open(&vault_path).unwrap();
      let mut statement = connection
        .prepare("SELECT COUNT(*) FROM schema_migrations")
        .unwrap();
      let count: u32 = statement.query_row([], |row| row.get(0)).unwrap();

      assert_eq!(count, u32::try_from(MIGRATIONS.len()).unwrap());
    }
  }
}
