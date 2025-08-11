mod builder;

use std::{collections::HashMap, path::PathBuf};

use builder::Builder;
use chrono::{DateTime, Utc};
use eyre::{Result, eyre};
use serde::{Deserialize, Serialize};

use crate::{config::Config, services::TomlLoaderService};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Index {
  pub canonical_id_to_location: HashMap<String, PathBuf>,
  pub last_modified: Option<DateTime<Utc>>,
  pub last_task_completed_at: Option<DateTime<Utc>>,
  pub last_task_created_at: Option<DateTime<Utc>>,
  pub next_task_id: u32,
  pub task_count_by_list: HashMap<String, usize>,
  pub total_projects: usize,
  pub total_tasks: usize,
  pub used_display_ids: HashMap<String, Vec<u32>>,
}

impl Default for Index {
  fn default() -> Self {
    Self {
      canonical_id_to_location: HashMap::new(),
      last_modified: None,
      last_task_completed_at: None,
      last_task_created_at: None,
      next_task_id: 1,
      task_count_by_list: HashMap::new(),
      total_projects: 0,
      total_tasks: 0,
      used_display_ids: HashMap::new(),
    }
  }
}

impl Index {
  pub fn load() -> Self {
    let index_path = Config::load().vault.path.join(".vault/index");

    if index_path.exists() {
      return TomlLoaderService::load_or_default(&index_path);
    }

    Builder::build().unwrap_or_default()
  }

  pub fn add_task(&mut self, task: &crate::entities::Task) -> Result<()> {
    self.total_tasks += 1;

    *self.task_count_by_list.entry(task.list_name.clone()).or_insert(0) += 1;

    self.last_task_created_at = Some(task.created.max(self.last_task_created_at.unwrap_or(task.created)));

    if let Some(completed) = task.completed {
      self.last_task_completed_at = Some(completed.max(self.last_task_completed_at.unwrap_or(completed)));
    }

    self.last_modified = Some(task.updated.max(self.last_modified.unwrap_or(task.updated)));

    let vault_path = Config::load().vault.path;
    let filename = if task.project_id.is_some() {
      format!("{}.md", task.display_id)
    } else {
      format!("{}.md", task.id)
    };
    let task_path = vault_path.join("tasks").join(&task.list_name).join(filename);
    self.canonical_id_to_location.insert(task.id.to_string(), task_path);

    self.save()
  }

  pub fn add_task_to_project(&mut self, project_id: &str, task_number: u32) -> Result<()> {
    self.used_display_ids.entry(project_id.to_string()).or_default().push(task_number);
    self.save()
  }

  pub fn find_task_location(&self, task_id: u32) -> Option<PathBuf> {
    self.canonical_id_to_location.get(&task_id.to_string()).cloned()
  }

  pub fn move_task(&mut self, task_id: u32, from_list: &str, to_list: &str, display_id: &str) -> Result<()> {
    if let Some(count) = self.task_count_by_list.get_mut(from_list) {
      *count = count.saturating_sub(1);
      if *count == 0 {
        self.task_count_by_list.remove(from_list);
      }
    }

    *self.task_count_by_list.entry(to_list.to_string()).or_insert(0) += 1;

    let vault_path = Config::load().vault.path;
    let filename = format!("{display_id}.md");
    let new_task_path = vault_path.join("tasks").join(to_list).join(filename);
    self.canonical_id_to_location.insert(task_id.to_string(), new_task_path);

    self.save()
  }

  pub fn next_project_task_number(&self, project_id: &str) -> u32 {
    let empty_vec = Vec::new();
    let used_numbers = self.used_display_ids.get(project_id).unwrap_or(&empty_vec);

    let mut next_num = 1;
    while used_numbers.contains(&next_num) {
      next_num += 1;
    }
    next_num
  }

  pub fn next_task_id(&mut self) -> u32 {
    let id = self.next_task_id;
    self.next_task_id += 1;
    self.save().unwrap();
    id
  }

  pub fn remove_task(&mut self, task: &crate::entities::Task) -> Result<()> {
    self.total_tasks = self.total_tasks.saturating_sub(1);

    if let Some(count) = self.task_count_by_list.get_mut(&task.list_name) {
      *count = count.saturating_sub(1);
      if *count == 0 {
        self.task_count_by_list.remove(&task.list_name);
      }
    }

    self.canonical_id_to_location.remove(&task.id.to_string());

    if let Some(project_id) = &task.project_id
      && let Some(num_str) = task.display_id.split('-').next_back()
      && let Ok(num) = num_str.parse::<u32>()
    {
      self.remove_task_from_project(project_id, num)?;
    }

    self.save()
  }

  pub fn remove_task_from_project(&mut self, project_id: &str, task_number: u32) -> Result<()> {
    if let Some(numbers) = self.used_display_ids.get_mut(project_id) {
      numbers.retain(|&n| n != task_number);
      if numbers.is_empty() {
        self.used_display_ids.remove(project_id);
      }
    }
    self.save()
  }

  pub fn save(&self) -> Result<()> {
    let index_path = Config::load().vault.path.join(".vault/index");
    std::fs::create_dir_all(index_path.parent().unwrap())?;
    let content = toml::to_string_pretty(self)?;
    std::fs::write(&index_path, content)?;
    Ok(())
  }

  pub fn update_task_location(&mut self, task_id: u32, list_name: &str, display_id: &str) -> Result<()> {
    let vault_path = Config::load().vault.path;
    let filename = format!("{display_id}.md");
    let task_path = vault_path.join("tasks").join(list_name).join(filename);
    self.canonical_id_to_location.insert(task_id.to_string(), task_path);
    self.save()
  }
}
