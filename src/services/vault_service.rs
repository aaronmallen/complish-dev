use std::path::Path;

use eyre::{Result, eyre};

pub struct VaultService;

impl VaultService {
  pub fn init(path: &Path) -> Result<()> {
    let task_dir = path.join("tasks");
    let project_dir = path.join("projects");

    for dir in &["today", "next", "someday"] {
      let path = task_dir.join(dir);
      std::fs::create_dir_all(&path)?;
    }

    std::fs::create_dir_all(&project_dir)?;

    Ok(())
  }

  pub fn mv(source: &Path, destination: &Path) -> Result<()> {
    if !source.exists() {
      return Err(eyre!("Source path does not exist: {}", source.display()));
    }

    if let Some(parent) = destination.parent() {
      std::fs::create_dir_all(parent)?;
    }

    if destination.exists() {
      return Err(eyre!("Destination already exists: {}", destination.display()));
    }

    if std::fs::rename(source, destination).is_err() {
      if source.is_file() {
        std::fs::copy(source, destination)?;
        std::fs::remove_file(source)?;
      } else if source.is_dir() {
        Self::copy_dir_recursive(source, destination)?;
        std::fs::remove_dir_all(source)?;
      }
    }

    Ok(())
  }

  fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    std::fs::create_dir_all(destination)?;

    for entry in std::fs::read_dir(source)? {
      let entry = entry?;
      let source_path = entry.path();
      let dest_path = destination.join(entry.file_name());

      if source_path.is_file() {
        std::fs::copy(&source_path, &dest_path)?;
      } else if source_path.is_dir() {
        Self::copy_dir_recursive(&source_path, &dest_path)?;
      }
    }

    Ok(())
  }
}
