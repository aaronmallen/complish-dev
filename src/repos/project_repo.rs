use std::path::PathBuf;

use eyre::{Result, eyre};

use crate::{
  config::Config,
  entities::{Project, Task},
  index::Index,
  services::{ProjectDocumentService, TaskDocumentService},
};

pub struct ProjectRepo {
  project_path: PathBuf,
}

impl ProjectRepo {
  pub fn new() -> Self {
    Self {
      project_path: Config::load().vault.path.join("projects"),
    }
  }

  pub fn add_task(&self, mut project: Project, mut task: Task) -> Result<Project> {
    let mut index = Index::load();

    if let Some(old_project_id) = &task.project_id {
      self.remove_task_from_old_project(&mut index, old_project_id, &task)?;
    }

    let project_id = project.id.clone();
    let task_number = index.next_project_task_number(&project_id);
    let new_display_id = format!("{project_id}-{task_number:02}");

    let old_display_id = task.display_id.clone();
    task.project_id = Some(project_id.clone());
    task.display_id.clone_from(&new_display_id);

    index.add_task_to_project(&project_id, task_number)?;

    Self::rename_task_file(&old_display_id, &new_display_id, &task)?;

    project.tasks.push(task);
    let contents = ProjectDocumentService::generate(project)?;
    std::fs::write(self.project_path.join(format!("{project_id}.md")), &contents)?;

    self.by_id(&project_id)
  }

  pub fn all(&self) -> Result<Vec<Project>> {
    let mut projects = Vec::new();

    if !self.project_path.exists() {
      return Ok(projects);
    }

    for file in self.project_path.read_dir()? {
      let file = file?;
      if file.path().extension().and_then(|s| s.to_str()) == Some("md") {
        match ProjectDocumentService::parse(&file.path()) {
          Ok(project) => projects.push(project),
          Err(e) => eprintln!("Warning: Failed to parse {}: {}", file.path().display(), e),
        }
      }
    }

    projects.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(projects)
  }

  pub fn by_id(&self, id: &str) -> Result<Project> {
    let path = self.project_path.join(format!("{id}.md"));

    if !path.exists() {
      return Err(eyre!("Project '{}' not found", id));
    }

    ProjectDocumentService::parse(&path)
  }

  pub fn by_name(&self, name: &str) -> Result<Project> {
    if !self.project_path.exists() {
      return Err(eyre!("Projects directory does not exist"));
    }

    for file in self.project_path.read_dir()? {
      let file = file?;
      if file.path().extension().and_then(|s| s.to_str()) == Some("md") {
        match ProjectDocumentService::parse(&file.path()) {
          Ok(project) => {
            if project.name == name {
              return Ok(project);
            }
          }
          Err(e) => eprintln!("Warning: Failed to parse {}: {}", file.path().display(), e),
        }
      }
    }

    Err(eyre!("Project '{}' not found", name))
  }

  pub fn create(&self, name: &str, description: Option<String>, mut id: Option<String>) -> Result<Project> {
    std::fs::create_dir_all(&self.project_path)?;

    if id.is_none() {
      id = Some(if name.len() >= 3 {
        name.chars().take(3).collect::<String>().to_lowercase()
      } else {
        name.to_lowercase()
      });
    }

    let project_id = id.unwrap();
    let path = self.project_path.join(format!("{project_id}.md"));

    if path.exists() {
      return Err(eyre!("Project with ID '{}' already exists", project_id));
    }

    let project = Project {
      description,
      id: project_id,
      name: name.to_string(),
      ..Project::default()
    };

    let contents = ProjectDocumentService::generate(project.clone())?;
    std::fs::write(&path, &contents)?;

    let mut index = Index::load();
    index.total_projects += 1;
    index.save()?;

    Ok(project)
  }

  pub fn delete(&self, id: &str) -> Result<()> {
    let path = self.project_path.join(format!("{id}.md"));

    if !path.exists() {
      return Err(eyre!("Project '{}' not found", id));
    }

    let project = ProjectDocumentService::parse(&path)?;
    let mut index = Index::load();

    for task in &project.tasks {
      Self::revert_task_to_canonical(&mut index, id, task)?;
    }

    std::fs::remove_file(&path)?;
    index.total_projects = index.total_projects.saturating_sub(1);
    index.save()?;

    Ok(())
  }

  pub fn remove_task(&self, project_id: &str, task_id: u32) -> Result<Project> {
    let project_path = self.project_path.join(format!("{project_id}.md"));

    if !project_path.exists() {
      return Err(eyre!("Project '{}' not found", project_id));
    }

    let mut project = ProjectDocumentService::parse(&project_path)?;

    let task_pos = project
      .tasks
      .iter()
      .position(|t| t.id == task_id)
      .ok_or_else(|| eyre!("Task {} not found in project {}", task_id, project_id))?;

    let removed_task = project.tasks.remove(task_pos);
    let mut index = Index::load();

    Self::revert_task_to_canonical(&mut index, project_id, &removed_task)?;

    let contents = ProjectDocumentService::generate(project.clone())?;
    std::fs::write(&project_path, &contents)?;

    Ok(project)
  }

  pub fn update(&self, mut project: Project) -> Result<Project> {
    let path = self.project_path.join(format!("{}.md", project.id));

    if !path.exists() {
      return Err(eyre!("Project '{}' not found", project.id));
    }

    project.updated = chrono::Utc::now();

    let contents = ProjectDocumentService::generate(project.clone())?;
    std::fs::write(&path, &contents)?;

    Ok(project)
  }

  fn extract_task_number_from_display_id(display_id: &str) -> Option<u32> {
    display_id.split('-').next_back()?.parse().ok()
  }

  fn remove_task_from_old_project(&self, index: &mut Index, old_project_id: &str, task: &Task) -> Result<()> {
    if let Some(old_num) = Self::extract_task_number_from_display_id(&task.display_id) {
      index.remove_task_from_project(old_project_id, old_num)?;
    }

    let old_project_path = self.project_path.join(format!("{old_project_id}.md"));
    if old_project_path.exists() {
      let mut old_project = ProjectDocumentService::parse(&old_project_path)?;
      if let Some(pos) = old_project.tasks.iter().position(|t| t.id == task.id) {
        old_project.tasks.remove(pos);
      }
      let old_project_contents = ProjectDocumentService::generate(old_project)?;
      std::fs::write(&old_project_path, &old_project_contents)?;
    }

    Ok(())
  }

  fn rename_task_file(old_display_id: &str, new_display_id: &str, task: &Task) -> Result<()> {
    let vault_path = Config::load().vault.path;
    let old_task_path = vault_path.join("tasks").join(&task.list_name).join(format!("{old_display_id}.md"));
    let new_task_path = vault_path.join("tasks").join(&task.list_name).join(format!("{new_display_id}.md"));

    if old_task_path.exists() {
      let task_content = TaskDocumentService::generate(task.clone())?;
      std::fs::write(&new_task_path, &task_content)?;
      std::fs::remove_file(&old_task_path)?;
    }

    Ok(())
  }

  fn revert_task_to_canonical(index: &mut Index, project_id: &str, task: &Task) -> Result<()> {
    if let Some(num) = Self::extract_task_number_from_display_id(&task.display_id) {
      index.remove_task_from_project(project_id, num)?;
    }

    let mut updated_task = task.clone();
    updated_task.project_id = None;
    updated_task.display_id = task.id.to_string();

    let vault_path = Config::load().vault.path;
    let old_task_path = vault_path.join("tasks").join(&task.list_name).join(format!("{}.md", task.display_id));
    let new_task_path = vault_path.join("tasks").join(&task.list_name).join(format!("{}.md", updated_task.display_id));

    if old_task_path.exists() {
      let task_content = TaskDocumentService::generate(updated_task)?;
      std::fs::write(&new_task_path, &task_content)?;
      std::fs::remove_file(&old_task_path)?;
    }

    Ok(())
  }
}
