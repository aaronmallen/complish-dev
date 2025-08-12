use std::process;

mod cli;
mod config;
mod entities;
mod index;
mod repos;
mod services;
mod ui;

fn main() {
  if let Err(e) = cli::run() {
    eprintln!("Error: {e}");
    process::exit(1);
  }
}
