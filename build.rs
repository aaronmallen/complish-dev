use std::process::Command;

fn main() {
  let build_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
  println!("cargo:rustc-env=BUILD_DATE={build_date}");

  let git_sha = Command::new("git")
    .args(["rev-parse", "--short", "HEAD"])
    .output()
    .map_or_else(|_| "unknown".to_string(), |output| String::from_utf8_lossy(&output.stdout).trim().to_string());
  println!("cargo:rustc-env=GIT_SHA={git_sha}");
  println!("cargo:rerun-if-changed=.git/HEAD");
  println!("cargo:rerun-if-changed=.git/refs/heads/");
}
