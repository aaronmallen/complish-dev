use clap::Args;
use color_eyre::Result;
use complish::{List as TaskList, Project, Task};

use crate::ui::TaskTable;

#[derive(Args, Debug)]
pub struct List {
  /// Filter tasks by list
  #[arg(long)]
  list: Option<String>,
  /// Filter tasks by project
  #[arg(long)]
  project: Option<String>,
  /// Filter tasks by tag
  #[arg(long, short = 't')]
  tag: Vec<String>,
}

impl List {
  pub fn run(self) -> Result<()> {
    let mut tasks = Task::all()?;

    if let Some(list_name) = self.list {
      let list = TaskList::find_by_name(list_name)?;
      let list_tasks = list.tasks()?;
      tasks.retain(|t| list_tasks.contains(t));
    }

    if let Some(key) = self.project {
      let project = Project::find_by_key(key)?;
      let project_tasks = project.tasks()?;
      tasks.retain(|t| project_tasks.contains(t));
    }

    if !self.tag.is_empty() {
      tasks.retain(|task| {
        if let Ok(task_tags) = task.tags() {
          let task_tag_labels: Vec<String> =
            task_tags.iter().map(|t| t.label().to_string()).collect();
          self.tag.iter().all(|tag| task_tag_labels.contains(tag))
        } else {
          false
        }
      });
    }

    TaskTable::new(tasks).render();
    Ok(())
  }
}
