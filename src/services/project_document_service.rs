use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use eyre::{Result, eyre};
use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};

use crate::{
  entities::{Project, Task},
  services::TaskDocumentService,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ProjectDocumentFrontMatter {
  pub completed: Option<DateTime<Utc>>,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}

pub struct ProjectDocumentService;

impl ProjectDocumentService {
  pub fn generate(project: Project) -> Result<String> {
    let front_matter = ProjectDocumentFrontMatter {
      completed: project.completed,
      created: project.created,
      updated: project.updated,
    };

    let front_matter_yaml = serde_yml::to_string(&front_matter)?;
    let tasks_section = Self::generate_tasks_section(&project.tasks);

    Ok(format!(
      "---\n{front_matter_yaml}---\n# {name}\n\n## Description\n\n{description}\n\n## Tasks\n\n{tasks}",
      front_matter_yaml = front_matter_yaml,
      name = project.name,
      description = project.description.unwrap_or_default(),
      tasks = tasks_section
    ))
  }

  pub fn parse(filepath: &PathBuf) -> Result<Project> {
    let (data, content) = Self::parse_front_matter(filepath)?;
    let (name, description, task_paths) = Self::parse_content(&content, filepath)?;

    let tasks: Result<Vec<Task>> = task_paths.iter().map(TaskDocumentService::parse).collect();

    let project_id = filepath.file_stem().unwrap().to_str().unwrap().to_string();

    Ok(Project {
      completed: data.completed,
      created: data.created,
      description,
      id: project_id,
      name,
      tasks: tasks?,
      updated: data.updated,
    })
  }

  fn extract_task_path(line: &str, project_filepath: &Path) -> Option<PathBuf> {
    if let Some(start) = line.find('(')
      && let Some(end) = line.find(')')
      && start < end
    {
      let relative_path = &line[start + 1..end];
      let project_dir = project_filepath.parent()?;
      let absolute_path = project_dir.join(relative_path);
      return Some(absolute_path);
    }
    None
  }

  fn generate_tasks_section(tasks: &[Task]) -> String {
    if tasks.is_empty() {
      return String::new();
    }

    tasks
      .iter()
      .map(|task| format!("- [{}](../tasks/{}/{}.md)", task.display_id, task.list_name, task.display_id))
      .collect::<Vec<_>>()
      .join("\n")
  }

  fn parse_content(content: &str, project_filepath: &Path) -> Result<(String, Option<String>, Vec<PathBuf>)> {
    let mut name = String::new();
    let mut description_lines = Vec::new();
    let mut task_paths = Vec::new();
    let mut current_section = None;

    for line in content.lines() {
      let trimmed = line.trim();

      if trimmed.starts_with("# ") {
        name = trimmed.replace("# ", "").trim().to_string();
        continue;
      }

      if trimmed == "## Description" {
        current_section = Some("description");
        continue;
      }

      if trimmed == "## Tasks" {
        current_section = Some("tasks");
        continue;
      }

      match current_section {
        Some("description") if !trimmed.is_empty() && !trimmed.starts_with("##") => {
          description_lines.push(trimmed.to_string());
        }
        Some("tasks") if trimmed.starts_with("- [") => {
          if let Some(task_path) = Self::extract_task_path(trimmed, project_filepath) {
            task_paths.push(task_path);
          }
        }
        _ => {}
      }
    }

    if name.is_empty() {
      return Err(eyre!("Invalid project document: missing name"));
    }

    let description = if description_lines.is_empty() {
      None
    } else {
      Some(description_lines.join("\n"))
    };

    Ok((name, description, task_paths))
  }

  fn parse_front_matter(filepath: &PathBuf) -> Result<(ProjectDocumentFrontMatter, String)> {
    let matter = Matter::<YAML>::new();
    let result = matter.parse::<ProjectDocumentFrontMatter>(&std::fs::read_to_string(filepath)?)?;
    let data = result.data.ok_or_else(|| eyre!("Invalid project document: missing front matter"))?;
    Ok((data, result.content))
  }
}
