use complish::{Task, TaskPriority, TaskWorkflowStatus};
use yansi::Paint;

use crate::ui::{color, text};

pub struct TaskDetail {
  task: Task,
}

impl TaskDetail {
  pub fn new(task: Task) -> Self {
    Self {
      task,
    }
  }

  pub fn render(&self) {
    println!();
    self.render_header();
    println!();
    self.render_metadata();
    self.render_description();
    self.render_notes();
    self.render_timestamps();
    println!();
  }

  fn format_priority(&self) -> String {
    match self.task.priority() {
      TaskPriority::Critical => text::warn(" ⚠  CRITICAL "),
      TaskPriority::High => "HIGH".bright_yellow().to_string(),
      TaskPriority::Medium => "MEDIUM".yellow().to_string(),
      TaskPriority::Low => "LOW".blue().to_string(),
      TaskPriority::Lowest => "LOWEST".dim().to_string(),
    }
  }

  fn format_status(&self) -> String {
    match self.task.workflow_status() {
      TaskWorkflowStatus::Blocked => "⊘".red().to_string(),
      TaskWorkflowStatus::Done => "●".green().to_string(),
      TaskWorkflowStatus::InProgress => "◐".yellow().to_string(),
      TaskWorkflowStatus::Todo => "○".dim().to_string(),
    }
  }

  fn render_header(&self) {
    let id_string = format!(" #{:?} ", self.task.sequence_id().unwrap_or(0))
      .bg(color::OFF_WHITE)
      .to_string();
    let status = self.format_status();
    print!("{} {} ", id_string, status);
    println!("{}", self.task.title().bold());
  }

  fn render_metadata(&self) {
    let priority_icon = self.format_priority();
    println!("{}    {}", "Priority:".dim(), priority_icon);

    if let Some(estimation) = self.task.estimation() {
      println!("{}  {}", "Estimation:".dim(), estimation);
    }

    self.render_project();
    self.render_tags();
  }

  fn render_description(&self) {
    if let Some(description) = self.task.description() {
      println!();
      println!("{}", "Description".dim());
      println!("{}", description);
    }
  }

  fn render_notes(&self) {
    if let Ok(notes) = self.task.notes()
      && !notes.is_empty()
    {
      println!();
      println!("{}", "Notes".dim());
      for note in notes {
        println!();
        println!(
          "  {} {}",
          "▸".dim(),
          note.created_at().format("%Y-%m-%d %H:%M").to_string().dim()
        );
        println!("  {}", note.content());
      }
    }
  }

  fn render_project(&self) {
    if let Some(project_id) = self.task.metadata().get("project_id")
      && let Ok(project) = complish::Project::find(project_id.clone())
    {
      println!("{}     {}", "Project:".dim(), project.name().bright_cyan());
    }
  }

  fn render_tags(&self) {
    if let Ok(tags) = self.task.tags()
      && !tags.is_empty()
    {
      let tag_labels: Vec<String> = tags.iter().map(|t| t.label().cyan().to_string()).collect();
      println!("{}        {}", "Tags:".dim(), tag_labels.join(", "));
    }
  }

  fn render_timestamps(&self) {
    println!();
    println!(
      "{}     {}",
      "Created:".dim(),
      self.task.created_at().format("%Y-%m-%d %H:%M")
    );
    println!(
      "{}     {}",
      "Updated:".dim(),
      self.task.updated_at().format("%Y-%m-%d %H:%M")
    );

    if let Some(completed_at) = self.task.completed_at() {
      println!(
        "{}   {}",
        "Completed:".dim(),
        completed_at.format("%Y-%m-%d %H:%M").to_string().green()
      );
    }
  }
}
