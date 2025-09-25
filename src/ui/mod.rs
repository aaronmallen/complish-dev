use std::io::{self, Write};

use color_eyre::{Result, eyre::eyre};

pub fn text_input(prompt: &str) -> Result<String> {
  println!("{} (Ctrl+D to finish):", prompt);

  let mut buffer = String::new();
  let mut consecutive_empty = 0;

  loop {
    io::stdout().flush()?;

    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
      Ok(0) => break,
      Ok(_) => {
        if line.trim().is_empty() {
          consecutive_empty += 1;
          if consecutive_empty >= 1 && !buffer.is_empty() {
            break;
          }
        } else {
          consecutive_empty = 0;
          buffer.push_str(&line);
        }
      }
      Err(e) => return Err(eyre!("Failed to read input: {}", e)),
    }
  }

  let trimmed = buffer.trim();
  if trimmed.is_empty() {
    Err(eyre!("No text was entered"))
  } else {
    Ok(trimmed.to_string())
  }
}
