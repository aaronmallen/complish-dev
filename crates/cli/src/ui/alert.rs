use yansi::Paint;

use super::color::*;

pub fn error(message: impl Into<String>) {
  eprintln!(
    "{} {}",
    " ERROR ".fg(OFF_WHITE).bg(ERROR_COLOR).bold(),
    message.into()
  )
}

pub fn info(message: impl Into<String>) {
  println!(
    "{} {}",
    " INFO ".fg(OFF_WHITE).bg(INFO_COLOR).bold(),
    message.into()
  )
}

pub fn success(message: impl Into<String>) {
  println!(
    "{} {}",
    " SUCCESS ".fg(OFF_WHITE).bg(SUCCESS_COLOR).bold(),
    message.into()
  )
}

pub fn warn(message: impl Into<String>) {
  eprintln!(
    "{} {}",
    " WARN ".fg(OFF_WHITE).bg(WARN_COLOR).bold(),
    message.into()
  )
}
