use std::process;

mod cli;

fn main() {
  if let Err(e) = cli::run() {
    eprintln!("Error: {e}");
    process::exit(1);
  }
}
