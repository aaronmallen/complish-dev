use std::path::Path;

use eyre::Result;

use crate::{config::Config, index::Index, services::TaskDocumentService};

pub struct Builder;

impl Builder {
  pub fn build() -> Result<Index> {
    let vault_path = Config::load().vault.path;
    let task_path = vault_path.join("tasks");
    let project_path = vault_path.join("projects");

    let mut index = Index::default();
    index = Self::scan_tasks(task_path.as_path(), index)?;
    index = Self::scan_projects(project_path.as_path(), index)?;

    index.save()?;
    Ok(index)
  }

  fn scan_projects(project_path: &Path, mut index: Index) -> Result<Index> {
    if !project_path.exists() {
      return Ok(index);
    }

    for entry in project_path.read_dir()? {
      let entry = entry?;
      if entry.path().extension().and_then(|s| s.to_str()) == Some("md") {
        index.total_projects += 1;
      }
    }

    Ok(index)
  }

  fn scan_tasks(task_path: &Path, mut index: Index) -> Result<Index> {
    if !task_path.exists() {
      return Ok(index);
    }

    for dir in task_path.read_dir()? {
      let dir = dir?;

      if dir.file_type()?.is_dir() {
        let list_name = dir.file_name().to_str().unwrap().to_string();
        let mut list_count = 0;

        for file in dir.path().read_dir()? {
          let file = file?;

          if file.path().extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
          }

          let task = TaskDocumentService::parse(&file.path())?;
          list_count += 1;
          index.total_tasks += 1;

          index.canonical_id_to_location.insert(task.id.to_string(), file.path());

          if index.last_modified.is_none_or(|last| last < task.updated) {
            index.last_modified = Some(task.updated);
          }

          if let Some(completed) = task.completed
            && index.last_task_completed_at.is_none_or(|last| last < completed)
          {
            index.last_task_completed_at = Some(completed);
          }

          if index.last_task_created_at.is_none_or(|last| last < task.created) {
            index.last_task_created_at = Some(task.created);
          }

          if index.next_task_id <= task.id {
            index.next_task_id = task.id + 1;
          }

          if let Some(project_id) = &task.project_id
            && let Some(num_str) = task.display_id.split('-').next_back()
            && let Ok(num) = num_str.parse::<u32>()
          {
            index.used_display_ids.entry(project_id.clone()).or_default().push(num);
          }
        }

        if list_count > 0 {
          index.task_count_by_list.insert(list_name, list_count);
        }
      }
    }

    Ok(index)
  }
}
