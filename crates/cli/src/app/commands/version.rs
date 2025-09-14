use std::env::consts::{ARCH, OS};

use clap::Args;
use color_eyre::Result;
use yansi::Paint;

// editorconfig-checker-disable
const LOGO: &str = r"
 ██████╗ ██████╗ ███╗   ███╗██████╗ ██╗     ██╗███████╗██╗  ██╗
██╔════╝██╔═══██╗████╗ ████║██╔══██╗██║     ██║██╔════╝██║  ██║
██║     ██║   ██║██╔████╔██║██████╔╝██║     ██║███████╗███████║
██║     ██║   ██║██║╚██╔╝██║██╔═══╝ ██║     ██║╚════██║██╔══██║
╚██████╗╚██████╔╝██║ ╚═╝ ██║██║     ███████╗██║███████║██║  ██║
 ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚══════╝╚═╝╚══════╝╚═╝  ╚═╝
";
// editorconfig-checker-enable

/// Display the version of complish
#[derive(Args, Debug)]
pub struct Version;

impl Version {
  pub fn run(&self) -> Result<()> {
    println!("{}", Self::brand());
    println!("{}", Self::author());
    println!("{}", Self::local_version());
    Ok(())
  }

  fn author() -> String {
    let author = format!(
      "{}{}",
      "by @".italic().bright_white(),
      "aaronmallen".rgb(234, 76, 132)
    );
    let spaces = " ".repeat(48);
    format!("{spaces}{author}")
  }

  fn brand() -> String {
    LOGO
      .chars()
      .map(|ch| match ch {
        '█' => ch.to_string().rgb(124, 199, 244).bold().to_string(),
        '╔' | '╗' | '╚' | '╝' | '║' | '═' | '╠' | '╣' | '╦' | '╩' | '╬' => {
          ch.to_string().rgb(250, 214, 71).bold().to_string()
        }
        _ => ch.to_string(),
      })
      .collect::<String>()
      .trim_end()
      .to_string()
  }

  fn local_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let build_date = env!("BUILD_DATE");
    let git_sha = env!("GIT_SHA");
    let platform = format!("{OS}-{ARCH}");
    format!("\nv{version} {platform} ({build_date} revision {git_sha})")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_brand_contains_logo() {
    let brand = Version::brand();

    assert!(brand.contains("COMPLISH") || brand.contains("█") || brand.contains("╔"));
    assert!(!brand.is_empty());
  }

  #[test]
  fn test_author_format() {
    let author = Version::author();

    assert!(author.contains("aaronmallen"));
    assert!(author.contains("by"));
  }

  #[test]
  fn test_local_version_format() {
    let version = Version::local_version();

    assert!(version.contains('v'));
    assert!(version.contains("revision"));
  }

  #[test]
  fn test_run_succeeds() {
    let version = Version;

    assert!(version.run().is_ok());
  }
}
