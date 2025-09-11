mod app;

use app::App;

fn main() {
  if let Err(e) = App::run() {
    eprintln!("Error: {}", e);
    std::process::exit(1)
  }
}
