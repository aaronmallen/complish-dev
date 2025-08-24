mod app;

use std::process;

use app::App;

fn main() {
  if let Err(e) = App::run() {
    eprintln!("Error: {e}");
    process::exit(1);
  }
}
