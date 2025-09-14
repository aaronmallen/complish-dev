mod app;
mod ui;

use std::process;

use app::App;
use ui::alert;
use yansi::Condition;

fn main() {
  yansi::whenever(Condition::TTY_AND_COLOR);

  if let Err(e) = App::run() {
    alert::error(e.to_string());
    process::exit(1)
  }
}
