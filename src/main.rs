mod game;
use std::{error::Error, time::Duration};

use cui::crossterm::run;
mod cui;

fn main() -> Result<(), Box<dyn Error>> {
    run(Duration::from_millis(500))?;
    Ok(())
}
