use std::path::PathBuf;

use chrono::{DateTime, Utc};
use eyre::{Result, eyre};
use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};

use crate::entities::{Priority, Task, TaskStatus};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TaskDocumentFrontMatter {
  pub completed: Option<DateTime<Utc>>,
  pub created: DateTime<Utc>,
  pub due_date: Option<DateTime<Utc>>,
  pub id: u32,
  pub priority: Priority,
  pub project_id: Option<String>,
  pub status: TaskStatus,
  pub list_name: String,
  pub updated: DateTime<Utc>,
}

pub struct TaskDocumentService;

impl TaskDocumentService {
  pub fn generate(task: Task) -> Result<String> {
    let front_matter = TaskDocumentFrontMatter {
      completed: task.completed,
      created: task.created,
      due_date: task.due_date,
      id: task.id,
      priority: task.priority,
      project_id: task.project_id,
      status: task.status,
      list_name: task.list_name,
      updated: task.updated,
    };

    let front_matter_yaml = serde_yml::to_string(&front_matter)?;

    Ok(format!(
      "---\n{front_matter_yaml}\n---\n# {subject}\n\n## Description\n\n{description}\n",
      front_matter_yaml = front_matter_yaml,
      subject = task.subject,
      description = task.description.unwrap_or_default(),
    ))
  }

  pub fn parse(filepath: &PathBuf) -> Result<Task> {
    let (data, content) = Self::parse_front_matter(filepath)?;

    let display_id = filepath.file_stem().unwrap().to_str().unwrap().to_string();
    let (subject, description) = Self::parse_content(&content)?;

    Ok(Task {
      completed: data.completed,
      created: data.created,
      description,
      display_id,
      due_date: data.due_date,
      id: data.id,
      priority: data.priority,
      project_id: data.project_id,
      status: data.status,
      subject,
      list_name: data.list_name,
      updated: data.updated,
    })
  }

  fn parse_content(content: &str) -> Result<(String, Option<String>)> {
    let mut description = None;
    let mut description_lines = Vec::new();
    let mut in_description = false;
    let mut subject = String::new();

    for line in content.lines() {
      let trimmed = line.trim();

      if trimmed.starts_with("# ") {
        subject = trimmed.replace("# ", "").trim().to_string();
      }

      if trimmed.starts_with("## Description") {
        in_description = true;
        continue;
      }

      if in_description {
        description_lines.push(line.to_string());
      }
    }

    if subject.is_empty() {
      return Err(eyre!("Invalid task document: missing subject"));
    }

    if !description_lines.is_empty() {
      description = Some(description_lines.join("\n"));
    }

    Ok((subject, description))
  }

  fn parse_front_matter(filepath: &PathBuf) -> Result<(TaskDocumentFrontMatter, String)> {
    let matter = Matter::<YAML>::new();
    let result = matter.parse::<TaskDocumentFrontMatter>(&std::fs::read_to_string(filepath)?)?;
    let data = result.data.ok_or_else(|| eyre!("Invalid task document: missing front matter"))?;
    Ok((data, result.content))
  }
}
