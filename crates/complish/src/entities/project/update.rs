use chrono::{DateTime, Utc};
use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::entities::project::update_status::UpdateStatus;

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct Update {
  #[get = "pub"]
  date: DateTime<Utc>,
  #[get = "pub"]
  description: Option<String>,
  #[get = "pub"]
  status: UpdateStatus,
}

impl Update {
  pub fn new(status: UpdateStatus) -> Self {
    Self {
      date: Utc::now(),
      description: None,
      status,
    }
  }

  #[must_use = "This method returns a new Update with the date set"]
  pub fn with_date(mut self, date: DateTime<Utc>) -> Self {
    self.date = date;
    self
  }

  #[must_use = "This method returns a new Update with the description set"]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod with_date {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_applies_the_date() {
      let date = Utc::now() - chrono::Duration::days(2);
      let update = Update::new(UpdateStatus::OnTrack).with_date(date);

      assert_eq!(update.date(), &date);
    }
  }

  mod with_description {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_applies_the_description() {
      let description = "This is a description";
      let update = Update::new(UpdateStatus::OnTrack).with_description(description);

      assert_eq!(
        update.description().clone().unwrap(),
        description.to_string()
      );
    }
  }
}
