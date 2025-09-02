use getset::Getters;

use super::Severity;

#[derive(Clone, Debug, Eq, Getters, PartialEq)]
pub struct Result {
  #[get = "pub"]
  context: String,
  #[get = "pub"]
  message: String,
  #[get = "pub"]
  severity: Severity,
}

impl Result {
  pub fn new(severity: Severity, message: impl Into<String>, context: impl Into<String>) -> Self {
    Self {
      context: context.into(),
      message: message.into(),
      severity,
    }
  }
}
