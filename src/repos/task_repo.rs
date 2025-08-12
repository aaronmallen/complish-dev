use std::path::PathBuf;

use chrono::Utc;
use eyre::{Result, eyre};

use crate::{config::Config, entities::Task, index::Index, services::TaskDocumentService};

pub struct TaskRepo {
  task_path: PathBuf,
}

impl TaskRepo {
  pub fn new() -> Self {
    Self {
      task_path: Config::load().vault.path.join("tasks"),
    }
  }

  pub fn by_id(id: u32) -> Result<Task> {
    let index = Index::load();
    let task_path = index.find_task_location(id).ok_or_else(|| eyre!("Task {} not found", id))?;

    TaskDocumentService::parse(&task_path)
  }

  pub fn delete(id: u32) -> Result<()> {
    let mut index = Index::load();
    let task_path = index.find_task_location(id).ok_or_else(|| eyre!("Task {} not found", id))?;
    let task = TaskDocumentService::parse(&task_path)?;

    std::fs::remove_file(&task_path)?;
    index.remove_task(&task)?;

    Ok(())
  }

  pub fn create(
    &self,
    subject: &str,
    list_name: &str,
    description: Option<String>,
    project_id: Option<&str>,
  ) -> Result<Task> {
    let mut index = Index::load();
    let id = index.next_task_id();

    let (display_id, final_project_id) = if let Some(proj_id) = project_id {
      let task_number = index.next_project_task_number(proj_id);
      index.add_task_to_project(proj_id, task_number)?;
      (format!("{proj_id}-{task_number:02}"), Some(proj_id.to_string()))
    } else {
      (id.to_string(), None)
    };

    let task = Task {
      id,
      display_id: display_id.clone(),
      project_id: final_project_id,
      subject: subject.to_string(),
      list_name: list_name.to_string(),
      description,
      ..Task::default()
    };

    let list_dir = self.task_path.join(list_name);
    std::fs::create_dir_all(&list_dir)?;

    let filename = format!("{display_id}.md");
    let task_path = list_dir.join(&filename);

    let contents = TaskDocumentService::generate(task.clone())?;
    std::fs::write(&task_path, &contents)?;

    index.add_task(&task)?;

    Ok(task)
  }

  pub fn update(&self, mut task: Task) -> Result<Task> {
    let mut index = Index::load();

    if let Some(old_task_path) = index.find_task_location(task.id)
      && old_task_path.exists()
    {
      std::fs::remove_file(&old_task_path)?;
    }

    task.updated = Utc::now();

    let list_dir = self.task_path.join(&task.list_name);
    std::fs::create_dir_all(&list_dir)?;

    let filename = if task.project_id.is_some() {
      format!("{}.md", task.display_id)
    } else {
      format!("{}.md", task.id)
    };

    let new_task_path = list_dir.join(&filename);
    let contents = TaskDocumentService::generate(task.clone())?;
    std::fs::write(&new_task_path, &contents)?;

    index.update_task_location(task.id, &task.list_name, &task.display_id)?;

    Ok(task)
  }
}
