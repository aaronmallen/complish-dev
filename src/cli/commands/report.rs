use std::collections::BTreeMap;

use chrono::{Local, NaiveDate, NaiveDateTime, TimeZone};
use clap::Args;
use color_eyre::Result;

use crate::{models::Accomplishment, parsers};

#[derive(Debug, Args)]
pub struct Report {
  /// Start date (e.g., 2025-01-15, 01/15/2025, today, yesterday)
  start: String,

  /// End date (e.g., 2025-01-31, 01/31/2025, today)
  end: String,
}

impl Report {
  pub fn run(&self) -> Result<()> {
    let start_date = parsers::parse_date(&self.start)?;
    let end_date = parsers::parse_date(&self.end)?;

    let start_local = start_date.and_hms_opt(0, 0, 0).unwrap();
    let end_local = end_date.and_hms_opt(23, 59, 59).unwrap();

    let start_utc = local_to_utc(start_local)?;
    let end_utc = local_to_utc(end_local)?;

    let accomplishments = Accomplishment::find_for_date_range(start_utc, end_utc)?;

    if accomplishments.is_empty() {
      println!(
        "No accomplishments found between {} and {}",
        self.start, self.end
      );
      return Ok(());
    }

    let mut by_date: BTreeMap<NaiveDate, Vec<Accomplishment>> = BTreeMap::new();

    for accomplishment in accomplishments {
      let local_dt = utc_to_local(*accomplishment.created_at())?;
      let date = local_dt.date();
      by_date.entry(date).or_default().push(accomplishment);
    }

    println!("# Accomplishments Report\n");
    println!("**Period:** {} to {}\n", self.start, self.end);

    for (date, items) in by_date {
      println!("## {}", date.format("%Y-%m-%d"));
      println!();
      for item in items {
        // Each accomplishment is one bullet, but preserve multiline structure
        let lines: Vec<&str> = item
          .content()
          .lines()
          .filter(|l| !l.trim().is_empty())
          .collect();
        if !lines.is_empty() {
          // First line gets the bullet
          println!("* {}", lines[0].trim());
          // Subsequent lines are indented continuations
          for line in lines.iter().skip(1) {
            println!("  {}", line.trim());
          }
        }
      }
      println!();
    }

    Ok(())
  }
}

fn local_to_utc(local: NaiveDateTime) -> Result<NaiveDateTime> {
  let local_dt = Local
    .from_local_datetime(&local)
    .single()
    .ok_or_else(|| color_eyre::eyre::eyre!("Invalid local datetime"))?;
  Ok(local_dt.naive_utc())
}

fn utc_to_local(utc: NaiveDateTime) -> Result<NaiveDateTime> {
  let utc_dt = chrono::Utc.from_utc_datetime(&utc);
  let local_dt = utc_dt.with_timezone(&Local);
  Ok(local_dt.naive_local())
}
