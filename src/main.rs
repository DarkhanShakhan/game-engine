mod board;
mod bot;
mod city;
mod game;
mod movement;
use std::{error::Error, time::Duration};

use board::Board;
use bot::Bot;
use ui::crossterm::run;
mod ui;
use crate::city::Owner;

fn main() -> Result<(), Box<dyn Error>> {
    run(Duration::from_millis(45))?;
    Ok(())
}

fn simulate() {
    let mut bot = Bot::new("./bot2/bot");
    let mut board = Board::default();
    board.set_city_owner("400-100", city::Owner::Player("p1".to_string()));
    board.current_player = "p1".to_string();
    for _ in 0..50 {
        board.ascii();
        bot.send_board(&board.to_string());
        // println!("{board}");
        if let Some(coords) = bot.get_move() {
            board.add_move(
                Owner::Player("p1".to_string()),
                &format!("{}-{}", coords.0, coords.1),
                &format!("{}-{}", coords.2, coords.3),
            );
        };
        board.tick();
    }
}
