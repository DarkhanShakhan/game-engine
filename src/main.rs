mod board;
mod bot;
mod city;
mod game;
mod movement;
use std::{error::Error, time::Duration};

use cui::crossterm::run;
mod cui;

fn main() -> Result<(), Box<dyn Error>> {
    run(Duration::from_millis(500))?;
    Ok(())
}
