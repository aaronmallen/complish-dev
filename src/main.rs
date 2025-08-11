use std::process;

mod cli;
mod config;

fn main() {
  if let Err(e) = cli::run() {
    eprintln!("Error: {e}");
    process::exit(1);
  }
}
