mod game;
use std::{error::Error, time::Duration};

use clap::Parser;
use cui::crossterm::run;
mod cui;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("{:?}", cli);
    run(
        Duration::from_millis(cli.tick_rate_ms),
        &cli.bot_1,
        &cli.bot_2,
    )?;
    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 500)]
    tick_rate_ms: u64,
    #[arg(long)]
    bot_1: String,
    #[arg(long)]
    bot_2: String,
}
