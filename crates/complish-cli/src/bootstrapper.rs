use std::path::PathBuf;

use complish::{
  Path,
  env::ComplishHomeResult,
  vault::{migrations, repo::Repo},
};
use eyre::Result;
use yansi::Condition;

pub struct Bootstrapper;

impl Bootstrapper {
  pub fn run() -> Result<Repo> {
    Self::configure_yansi();
    let vault_path = Self::get_vault_path();
    Self::run_migrations(&vault_path)?;
    Repo::new(vault_path)
  }

  fn configure_yansi() {
    yansi::whenever(Condition::TTY_AND_COLOR);
  }

  fn get_vault_path() -> PathBuf {
    let home_result = complish::env::complish_home();

    match home_result {
      ComplishHomeResult::Found(home) => home,
      ComplishHomeResult::NotAbsolute(path) => {
        eprintln!(
          "WARNING: $COMPLISH_HOME is not an absolute path: {}",
          path.display()
        );
        Self::get_default_vault_path()
      }
      ComplishHomeResult::NotSet => Self::get_default_vault_path(),
    }
  }

  fn get_default_vault_path() -> PathBuf {
    let complish_path = Path::new(Path::default_home_dir().unwrap());
    complish_path.vault_file()
  }

  fn run_migrations(vault_path: &PathBuf) -> eyre::Result<()> {
    migrations::run(vault_path)
  }
}
