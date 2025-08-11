use clap::Args;
use eyre::Result;

/// Display the version of complish
#[derive(Args, Debug)]
pub struct Version;

impl Version {
  pub fn run(&self) -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let build_date = env!("BUILD_DATE");
    let git_sha = env!("GIT_SHA");
    let platform = format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS);

    println!("complish v{version} ({build_date} revision {git_sha}) [{platform}]");
    Ok(())
  }
}
