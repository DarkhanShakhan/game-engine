mod game;
use std::{error::Error, time::Duration};

use clap::Parser;
use cui::crossterm::run;
mod cui;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    run(
        Duration::from_millis(cli.tick_rate_ms),
        &cli.player1,
        &cli.player2,
        &cli.map,
    )?;
    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 500)]
    tick_rate_ms: u64,
    #[arg(long, default_value_t = String::from("./bots/bot"))]
    player1: String,
    #[arg(long, default_value_t = String::from("./bots/bot"))]
    player2: String,
    #[arg(short,long, default_value_t = String::from("map1"))]
    map: String,
}
