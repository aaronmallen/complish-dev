use chrono::{DateTime, Utc};
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct List {
  #[get = "pub"]
  created_at: DateTime<Utc>,
  #[get = "pub"]
  id: u32,
  #[get = "pub"]
  name: String,
  #[get = "pub"]
  updated_at: DateTime<Utc>,
}
