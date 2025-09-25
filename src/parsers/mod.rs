use chrono::NaiveDate;
use color_eyre::{Result, eyre::eyre};

/// Parses a date string in various common formats
pub fn parse_date(input: &str) -> Result<NaiveDate> {
  let formats = [
    "%Y-%m-%d",  // 2025-01-15
    "%Y/%m/%d",  // 2025/01/15
    "%m-%d-%Y",  // 01-15-2025
    "%m/%d/%Y",  // 01/15/2025
    "%d-%m-%Y",  // 15-01-2025
    "%d/%m/%Y",  // 15/01/2025
    "%Y%m%d",    // 20250115
    "%b %d, %Y", // Jan 15, 2025
    "%B %d, %Y", // January 15, 2025
    "%d %b %Y",  // 15 Jan 2025
    "%d %B %Y",  // 15 January 2025
    "%m-%d-%y",  // 01-15-25
    "%m/%d/%y",  // 01/15/25
    "%y-%m-%d",  // 25-01-15
    "%y/%m/%d",  // 25/01/15
  ];

  for format in formats {
    if let Ok(date) = NaiveDate::parse_from_str(input, format) {
      return Ok(date);
    }
  }

  let normalized = input.trim().to_lowercase();

  if normalized == "today" {
    return Ok(chrono::Local::now().naive_local().date());
  }
  if normalized == "yesterday" {
    return Ok(chrono::Local::now().naive_local().date() - chrono::Duration::days(1));
  }
  if normalized == "tomorrow" {
    return Ok(chrono::Local::now().naive_local().date() + chrono::Duration::days(1));
  }

  Err(eyre!(
    "Could not parse '{}' as a date. Try formats like: YYYY-MM-DD, MM/DD/YYYY, or 'today'",
    input
  ))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_date_formats() {
    assert!(parse_date("2025-01-15").is_ok());
    assert!(parse_date("2025/01/15").is_ok());

    assert!(parse_date("01-15-2025").is_ok());
    assert!(parse_date("01/15/2025").is_ok());

    assert!(parse_date("15-01-2025").is_ok());
    assert!(parse_date("15/01/2025").is_ok());

    assert!(parse_date("20250115").is_ok());

    assert!(parse_date("Jan 15, 2025").is_ok());
    assert!(parse_date("January 15, 2025").is_ok());
    assert!(parse_date("15 Jan 2025").is_ok());

    assert!(parse_date("today").is_ok());
    assert!(parse_date("yesterday").is_ok());
    assert!(parse_date("tomorrow").is_ok());

    assert!(parse_date("not a date").is_err());
  }
}
