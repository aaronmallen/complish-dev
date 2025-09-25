use clap::Args;
use color_eyre::Result;

use crate::{models::Accomplishment, ui};

#[derive(Args, Debug)]
pub struct Journal;

impl Journal {
  pub fn run(&self) -> Result<()> {
    let content = ui::text_input("What did you accomplish?")?;
    let accomplishment = Accomplishment::new(content);
    accomplishment.save()?;

    Ok(())
  }
}
