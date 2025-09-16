use chrono::NaiveDateTime;
use color_eyre::eyre::eyre;

pub fn naive_datetime(s: &str) -> color_eyre::Result<NaiveDateTime> {
  let formats = [
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%d %H:%M",
    "%Y-%m-%dT%H:%M",
    "%Y/%m/%d %H:%M:%S",
    "%Y/%m/%d %H:%M",
    "%d-%m-%Y %H:%M:%S",
    "%d-%m-%Y %H:%M",
    "%d/%m/%Y %H:%M:%S",
    "%d/%m/%Y %H:%M",
    "%Y-%m-%d",
    "%Y/%m/%d",
    "%d-%m-%Y",
    "%d/%m/%Y",
  ];

  for format in formats {
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, format) {
      return Ok(dt);
    }
    if !format.contains("%H")
      && let Ok(date) = chrono::NaiveDate::parse_from_str(s, format)
    {
      return Ok(date.and_hms_opt(0, 0, 0).unwrap());
    }
  }

  Err(eyre!(
    "Invalid datetime format. Examples: '2024-12-25 14:30:00', '2024-12-25T14:30', '25/12/2024 14:30', '2024-12-25'"
  ))
}
