use std::{collections::HashSet, fs, result::Result as StdResult};

use eyre::{Context, Result, eyre};
use rusqlite::Connection;

use crate::config::Config;

const MIGRATIONS: &[(&str, &str)] = &[
  (
    "0001_enable_foreign_keys",
    include_str!("migrations/0001_enable_foreign_keys.sql"),
  ),
  (
    "0002_create_tags",
    include_str!("migrations/0002_create_tags.sql"),
  ),
  (
    "0003_create_projects",
    include_str!("migrations/0003_create_projects.sql"),
  ),
  (
    "0004_create_project_updates",
    include_str!("migrations/0004_create_project_updates.sql"),
  ),
  (
    "0005_create_project_tags",
    include_str!("migrations/0005_create_project_tags.sql"),
  ),
  (
    "0006_create_tasks",
    include_str!("migrations/0006_create_tasks.sql"),
  ),
  (
    "0007_create_task_notes",
    include_str!("migrations/0007_create_task_notes.sql"),
  ),
  (
    "0008_create_task_work_logs",
    include_str!("migrations/0008_create_task_work_logs.sql"),
  ),
  (
    "0009_create_task_relationships",
    include_str!("migrations/0009_create_task_relationships.sql"),
  ),
  (
    "0010_create_task_tags",
    include_str!("migrations/0010_create_task_tags.sql"),
  ),
  (
    "0011_create_journal_entries",
    include_str!("migrations/0011_create_journal_entries.sql"),
  ),
  (
    "0012_create_sprints",
    include_str!("migrations/0012_create_sprints.sql"),
  ),
  (
    "0013_create_sprint_tasks",
    include_str!("migrations/0013_create_sprint_tasks.sql"),
  ),
];

pub fn run(config: Config) -> Result<()> {
  let store_file = config.settings().core().store_file();
  if let Some(parent) = store_file.parent() {
    fs::create_dir_all(parent).with_context(|| {
      eyre!(
        "Failed to create data store directory: {}",
        parent.display()
      )
    })?;
  }

  let connection = Connection::open(store_file)?;
  run_with_connection(&connection)
}

pub fn run_with_connection(connection: &Connection) -> Result<()> {
  create_migrations_table(connection)?;
  run_migrations(connection)?;

  Ok(())
}

fn create_migrations_table(connection: &Connection) -> Result<()> {
  connection.execute(
    "CREATE TABLE IF NOT EXISTS schema_migrations (filename TEXT PRIMARY KEY)",
    [],
  )?;

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
    return Ok(());
  }

  for (filename, sql) in pending_migrations {
    execute_migration(connection, sql)?;
    update_schema_version(connection, filename)?;
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
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn it_creates_the_store_file() {
      let temp_dir = TempDir::new().unwrap();
      let data_home = temp_dir.path().join("data/complish");
      let config_home = temp_dir.path().join("config/complish");
      let mock_config = format!("[core]\ndata_home = \"{}\"", data_home.display());
      fs::create_dir_all(&config_home).unwrap();
      fs::write(config_home.join("config"), mock_config).unwrap();
      let config = Config::load_from(&config_home).unwrap();
      run(config).unwrap();

      assert!(data_home.join("store").exists());
    }

    #[test]
    fn it_creates_the_schema_migrations_table_and_runs_migrations() {
      let connection = Connection::open_in_memory().unwrap();
      run_with_connection(&connection).unwrap();

      let mut statement = connection
        .prepare("SELECT COUNT(*) FROM schema_migrations")
        .unwrap();
      let count: u32 = statement.query_row([], |row| row.get(0)).unwrap();

      assert_eq!(count, u32::try_from(MIGRATIONS.len()).unwrap());
    }

    #[test]
    fn it_is_idempotent() {
      let connection = Connection::open_in_memory().unwrap();
      run_with_connection(&connection).unwrap();
      run_with_connection(&connection).unwrap();

      let mut statement = connection
        .prepare("SELECT COUNT(*) FROM schema_migrations")
        .unwrap();
      let count: u32 = statement.query_row([], |row| row.get(0)).unwrap();

      assert_eq!(count, u32::try_from(MIGRATIONS.len()).unwrap());
    }
  }
}
