mod app;
mod bootstrapper;

use std::process;

use app::App;
use bootstrapper::Bootstrapper;

fn main() {
  let repo = match Bootstrapper::run() {
    Ok(repo) => repo,
    Err(e) => {
      eprintln!("Bootstrap Error: {e}");
      process::exit(1);
    }
  };

  if let Err(e) = App::run() {
    eprintln!("Error: {e}");
    process::exit(1);
  }
}
