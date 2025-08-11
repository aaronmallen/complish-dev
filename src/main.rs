use std::process;

mod cli;
mod config;
mod entities;

fn main() {
  if let Err(e) = cli::run() {
    eprintln!("Error: {e}");
    process::exit(1);
  }
}
