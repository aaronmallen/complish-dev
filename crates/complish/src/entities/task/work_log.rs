use chrono::{DateTime, Utc};
use cuid::cuid2;
use getset::{CloneGetters, Getters};
use serde::{Deserialize, Serialize};

#[derive(Clone, CloneGetters, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct WorkLog {
  #[get_clone = "pub"]
  ended_at: Option<DateTime<Utc>>,
  #[get = "pub"]
  id: String,
  #[get_clone = "pub"]
  note: Option<String>,
  #[get_clone = "pub"]
  source: Option<String>,
  #[get = "pub"]
  started_at: DateTime<Utc>,
}

impl Default for WorkLog {
  fn default() -> Self {
    Self::new()
  }
}

impl WorkLog {
  pub fn new() -> Self {
    Self {
      ended_at: None,
      id: cuid2(),
      note: None,
      source: None,
      started_at: Utc::now(),
    }
  }

  pub fn stop(&mut self) {
    self.ended_at = Some(Utc::now());
  }

  #[must_use = "This method returns a new WorkLog with the end date set"]
  pub fn with_end_date(mut self, ended_at: DateTime<Utc>) -> Self {
    self.ended_at = Some(ended_at);
    self
  }

  #[must_use = "This method returns a new WorkLog with the note set"]
  pub fn with_note(mut self, note: impl Into<String>) -> Self {
    self.note = Some(note.into());
    self
  }

  #[must_use = "This method returns a new WorkLog with the source set"]
  pub fn with_source(mut self, source: impl Into<String>) -> Self {
    self.source = Some(source.into());
    self
  }

  #[must_use = "This method returns a new WorkLog with the start date set"]
  pub fn with_start_date(mut self, started_at: DateTime<Utc>) -> Self {
    self.started_at = started_at;
    self
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod stop {
    use super::*;

    #[test]
    fn it_sets_ended_at() {
      let mut work_log = WorkLog::new();
      work_log.stop();

      assert!(work_log.ended_at.is_some());
      assert!(work_log.ended_at.unwrap().timestamp() > 0);
    }
  }

  mod with_end_date {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_work_log_with_the_end_date() {
      let date = Utc::now();
      let work_log = WorkLog::new().with_end_date(date);

      assert_eq!(work_log.ended_at(), Some(date));
    }
  }

  mod with_note {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_work_log_with_the_note() {
      let note = "This is a note";
      let work_log = WorkLog::new().with_note(note);

      assert_eq!(work_log.note(), Some(note.to_string()));
    }
  }

  mod with_source {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_work_log_with_the_source() {
      let source = "git";
      let work_log = WorkLog::new().with_source(source);

      assert_eq!(work_log.source(), Some(source.to_string()));
    }
  }

  mod with_start_date {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_a_new_work_log_with_the_start_date() {
      let date = Utc::now() - chrono::Duration::days(2);
      let work_log = WorkLog::new().with_start_date(date);

      assert_eq!(work_log.started_at(), &date);
    }
  }
}
