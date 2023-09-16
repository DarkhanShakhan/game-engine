mod board;
mod bot;
mod city;
mod movement;

use board::Board;
use bot::Bot;

use crate::city::Owner;

fn main() {
    let mut bot = Bot::new("./bot1/bot");
    let mut board = Board::default();
    board.set_city_owner("400-100", city::Owner::Player("p1".to_string()));
    board.current_player = "p1".to_string();
    for tick in 0..25 {
        board.tick = tick;
        bot.send_board(&board.to_string());
        println!("{board}");
        if let Some(coords) = bot.get_move() {
            println!("coords {:?}", coords);
            board.add_move(
                Owner::Player("p1".to_string()),
                &format!("{}-{}", coords.0, coords.1),
                &format!("{}-{}", coords.2, coords.3),
            );
        };
        board.tick();
    }
}
