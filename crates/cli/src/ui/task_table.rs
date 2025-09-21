use std::io::{self, Write};

use complish::{Task, TaskPriority, TaskWorkflowStatus};
use yansi::Paint;

use crate::ui::{color, text};

enum UserAction {
  Next,
  Previous,
  Quit,
  Unknown,
}

pub struct TaskTable {
  tasks: Vec<Task>,
  page_size: usize,
}

impl TaskTable {
  pub fn new(tasks: Vec<Task>) -> Self {
    Self {
      tasks,
      page_size: Self::get_page_size(),
    }
  }

  pub fn render(&self) {
    if self.tasks.is_empty() {
      println!("No tasks found matching the filters.");
      return;
    }

    Self::enter_fullscreen_mode();
    let total_pages = self.tasks.len().div_ceil(self.page_size);
    let mut current_page = 0;

    let result: Result<(), ()> = loop {
      Self::render_page(&self.tasks, current_page, self.page_size, total_pages);

      match Self::get_user_input() {
        UserAction::Next => {
          if current_page + 1 < total_pages {
            current_page += 1;
          }
        }
        UserAction::Previous => {
          current_page = current_page.saturating_sub(1);
        }
        UserAction::Quit => break Ok(()),
        UserAction::Unknown => {}
      }
    };

    Self::exit_fullscreen_mode();
    result.unwrap_or(());
  }

  fn enter_fullscreen_mode() {
    print!("\x1B[?1049h\x1B[?7l");
    io::stdout().flush().unwrap();
  }

  fn exit_fullscreen_mode() {
    print!("\x1B[?1049l\x1B[?7h");
    io::stdout().flush().unwrap();
  }

  fn format_estimation(task: &Task) -> String {
    task
      .estimation()
      .map(|est| est.to_string())
      .unwrap_or_else(|| "--".to_string())
  }

  fn format_priority(priority: &TaskPriority) -> String {
    let bar1 = "▂";
    let bar2 = "▄";
    let bar3 = "▆";
    let bar4 = "█";

    match priority {
      TaskPriority::Critical => text::warn(" ⚠  "),
      TaskPriority::High => format!(
        "{}{}{}{}",
        bar1.white(),
        bar2.white(),
        bar3.white(),
        bar4.white()
      ),
      TaskPriority::Medium => format!(
        "{}{}{}{}",
        bar1.white(),
        bar2.white(),
        bar3.white(),
        bar4.dim()
      ),
      TaskPriority::Low => format!(
        "{}{}{}{}",
        bar1.white(),
        bar2.white(),
        bar3.dim(),
        bar4.dim()
      ),
      TaskPriority::Lowest => format!("{}{}{}{}", bar1.white(), bar2.dim(), bar3.dim(), bar4.dim()),
    }
  }

  fn format_status(status: &TaskWorkflowStatus) -> &'static str {
    match status {
      TaskWorkflowStatus::Blocked => "⊘",
      TaskWorkflowStatus::Done => "●",
      TaskWorkflowStatus::InProgress => "◐",
      TaskWorkflowStatus::Todo => "○",
    }
  }

  fn format_tags(task: &Task) -> String {
    task
      .tags()
      .map(|tags| {
        tags
          .iter()
          .map(|t| t.label().to_string())
          .collect::<Vec<_>>()
          .join(", ")
      })
      .unwrap_or_default()
  }

  fn get_page_size() -> usize {
    if let Ok((_, height)) = crossterm::terminal::size() {
      (height as usize).saturating_sub(4)
    } else {
      20
    }
  }

  fn get_terminal_height() -> usize {
    if let Ok((_, height)) = crossterm::terminal::size() {
      height as usize
    } else {
      24
    }
  }

  fn get_terminal_width() -> usize {
    if let Ok((width, _)) = crossterm::terminal::size() {
      width as usize
    } else {
      100
    }
  }

  fn get_user_input() -> UserAction {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
      "n" | "next" => UserAction::Next,
      "p" | "prev" | "previous" => UserAction::Previous,
      "q" | "quit" | "" => UserAction::Quit,
      _ => UserAction::Unknown,
    }
  }

  fn print_header() {
    let terminal_width = Self::get_terminal_width();

    let fixed_width = 8 + 5 + 7;
    let remaining_width = terminal_width.saturating_sub(fixed_width);
    let title_width = (remaining_width * 3 / 4).max(20);
    let tags_width = remaining_width.saturating_sub(title_width);

    print!("{:<8}", "");
    print!("{:<5}", "");
    print!("{:<width$}", "Title", width = title_width);
    print!("{:<7}", "Est");
    println!("{:<width$}", "Tags", width = tags_width);
    println!("{}", "-".repeat(terminal_width));
  }

  fn print_pagination_controls(current_page: usize, total_pages: usize, total_tasks: usize) {
    let terminal_height = Self::get_terminal_height();

    print!("\x1B[{};1H", terminal_height - 2);
    println!(
      "Page {} of {} | {} tasks total",
      current_page + 1,
      total_pages,
      total_tasks
    );
    print!("\x1B[{};1H", terminal_height - 1);
    println!("[n]ext, [p]revious, [q]uit");
    print!("\x1B[{};1H", terminal_height);
    print!("> ");
    io::stdout().flush().unwrap();
  }

  fn print_task_row(task: &Task) {
    let terminal_width = Self::get_terminal_width();
    let id = task.sequence_id().unwrap_or(0);
    let status = Self::format_status(task.workflow_status());
    let priority = Self::format_priority(task.priority());
    let estimation = Self::format_estimation(task);

    let fixed_width = 8 + 5 + 7;
    let remaining_width = terminal_width.saturating_sub(fixed_width);
    let title_width = (remaining_width * 3 / 4).max(20);
    let tags_width = remaining_width.saturating_sub(title_width);

    let title = Self::truncate_text(task.title(), title_width.saturating_sub(1));
    let tags = Self::truncate_text(&Self::format_tags(task), tags_width.saturating_sub(1));

    let id_string = format!(" #{} ", id).bg(color::OFF_WHITE).to_string();

    print!("{} {} ", id_string, status);
    print!("{} ", priority);
    print!("{:<width$}", title, width = title_width);
    print!("{:<7}", estimation);
    println!("{:<width$}", tags, width = tags_width);
  }

  fn render_page(tasks: &[Task], current_page: usize, page_size: usize, total_pages: usize) {
    let start = current_page * page_size;
    let end = (start + page_size).min(tasks.len());
    let page_tasks = &tasks[start..end];

    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
    Self::print_header();

    for task in page_tasks {
      Self::print_task_row(task);
    }

    Self::print_pagination_controls(current_page, total_pages, tasks.len());
  }

  fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() > max_length {
      format!("{}...", &text[..max_length])
    } else {
      text.to_string()
    }
  }
}
