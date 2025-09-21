use clap::Args;
use color_eyre::Result;
use complish::{List as TaskList, Project, Task};

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

    println!(
      "{:<12} {:<8} {:<10} {:<40} {:<20}",
      "ID", "Status", "Priority", "Title", "Tags"
    );
    println!("{}", "-".repeat(100));

    if tasks.is_empty() {
      println!("No tasks found matching the filters.");
    } else {
      for task in tasks {
        let id = task.sequence_id().unwrap_or(0);
        let status = format!("{:?}", task.workflow_status());
        let priority = format!("{:?}", task.priority());
        let title = if task.title().len() > 37 {
          format!("{}...", &task.title()[..37])
        } else {
          task.title().to_string()
        };

        let tags = if let Ok(task_tags) = task.tags() {
          task_tags
            .iter()
            .map(|t| t.label().to_string())
            .collect::<Vec<_>>()
            .join(", ")
        } else {
          String::new()
        };

        let tags_display = if tags.len() > 18 {
          format!("{}...", &tags[..18])
        } else {
          tags
        };

        println!(
          "{:<12} {:<8} {:<10} {:<40} {:<20}",
          id, status, priority, title, tags_display
        );
      }
    }

    Ok(())
  }
}
