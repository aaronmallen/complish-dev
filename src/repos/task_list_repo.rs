use std::path::PathBuf;

use eyre::{Result, eyre};

use crate::{
  config::Config,
  entities::{Task, TaskList},
  index::Index,
  services::TaskDocumentService,
};

pub struct TaskListRepo {
  task_path: PathBuf,
}

impl TaskListRepo {
  pub fn new() -> Self {
    Self {
      task_path: Config::load().vault.path.join("tasks"),
    }
  }

  pub fn add_task(&self, task_list: &TaskList, mut task: Task) -> Result<TaskList> {
    let mut index = Index::load();

    let old_file_path = self.task_path.join(&task.list_name).join(format!("{}.md", task.display_id));

    task.list_name.clone_from(&task_list.name);

    let filename = if task.project_id.is_some() {
      format!("{}.md", task.display_id)
    } else {
      format!("{}.md", task.id)
    };

    let new_dir = self.task_path.join(&task_list.name);
    let new_file_path = new_dir.join(&filename);

    if old_file_path.exists() {
      std::fs::remove_file(&old_file_path)?;
    }

    std::fs::create_dir_all(&new_dir)?;

    let contents = TaskDocumentService::generate(task.clone())?;
    std::fs::write(&new_file_path, &contents)?;

    index.move_task(&task.list_name, &task_list.name)?;

    self.by_name(&task_list.name)
  }

  pub fn all(&self) -> Result<Vec<TaskList>> {
    let mut task_lists = Vec::new();

    if !self.task_path.exists() {
      return Ok(task_lists);
    }

    for entry in self.task_path.read_dir()? {
      let entry = entry?;
      if entry.file_type()?.is_dir() {
        let list_name = entry.file_name().to_string_lossy().to_string();
        match self.by_name(&list_name) {
          Ok(task_list) => task_lists.push(task_list),
          Err(e) => eprintln!("Warning: Failed to load task list {list_name}: {e}"),
        }
      }
    }

    task_lists.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(task_lists)
  }

  pub fn by_name(&self, name: &str) -> Result<TaskList> {
    let path = self.task_path.join(name);

    if !path.exists() {
      return Err(eyre!("Task list '{}' does not exist", name));
    }

    let mut tasks = Vec::new();
    for file in path.read_dir()? {
      let file = file?;
      if file.path().extension().and_then(|s| s.to_str()) == Some("md") {
        match TaskDocumentService::parse(&file.path()) {
          Ok(task) => tasks.push(task),
          Err(e) => eprintln!("Warning: Failed to parse {}: {}", file.path().display(), e),
        }
      }
    }

    tasks.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(TaskList {
      name: name.to_string(),
      tasks,
    })
  }

  pub fn create(&self, name: &str) -> Result<TaskList> {
    let path = self.task_path.join(name);

    if path.exists() {
      return Err(eyre!("Task list '{}' already exists", name));
    }

    std::fs::create_dir_all(&path)?;

    Ok(TaskList {
      name: name.to_string(),
      tasks: Vec::new(),
    })
  }

  pub fn create_task(&self, list_name: &str, subject: &str, description: Option<String>) -> Result<Task> {
    let mut index = Index::load();
    let task_id = index.next_task_id();

    let task = Task {
      id: task_id,
      display_id: task_id.to_string(),
      subject: subject.to_string(),
      description,
      list_name: list_name.to_string(),
      ..Task::default()
    };

    let list_dir = self.task_path.join(list_name);
    std::fs::create_dir_all(&list_dir)?;

    let filename = format!("{}.md", task.id);
    let task_path = list_dir.join(&filename);

    let contents = TaskDocumentService::generate(task.clone())?;
    std::fs::write(&task_path, &contents)?;

    index.add_task(&task)?;

    Ok(task)
  }

  pub fn delete(&self, name: &str) -> Result<()> {
    let path = self.task_path.join(name);

    if !path.exists() {
      return Err(eyre!("Task list '{}' not found", name));
    }

    let task_list = self.by_name(name)?;
    let mut index = Index::load();

    for task in &task_list.tasks {
      index.remove_task(task)?;
    }

    std::fs::remove_dir_all(&path)?;

    Ok(())
  }

  pub fn delete_task(&self, list_name: &str, task_id: u32) -> Result<()> {
    let list_path = self.task_path.join(list_name);

    if !list_path.exists() {
      return Err(eyre!("Task list '{}' not found", list_name));
    }

    let task_list = self.by_name(list_name)?;
    let task = task_list
      .tasks
      .iter()
      .find(|t| t.id == task_id)
      .ok_or_else(|| eyre!("Task {} not found in list '{}'", task_id, list_name))?
      .clone();

    let filename = if task.project_id.is_some() {
      format!("{}.md", task.display_id)
    } else {
      format!("{}.md", task.id)
    };

    let task_path = list_path.join(&filename);

    if task_path.exists() {
      std::fs::remove_file(&task_path)?;
    }

    let mut index = Index::load();
    index.remove_task(&task)?;

    Ok(())
  }

  pub fn update_task(&self, mut task: Task) -> Result<Task> {
    let list_path = self.task_path.join(&task.list_name);

    if !list_path.exists() {
      return Err(eyre!("Task list '{}' not found", task.list_name));
    }

    task.updated = chrono::Utc::now();

    let filename = if task.project_id.is_some() {
      format!("{}.md", task.display_id)
    } else {
      format!("{}.md", task.id)
    };

    let task_path = list_path.join(&filename);
    let contents = TaskDocumentService::generate(task.clone())?;
    std::fs::write(&task_path, &contents)?;

    Ok(task)
  }
}
