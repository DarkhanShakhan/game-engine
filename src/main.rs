mod board;
mod bot;
mod city;
mod movement;

use board::Board;
use bot::Bot;
use city::City;

use crate::city::Owner;

fn main() {
    let mut bot = Bot::new("./bot1/bot.py");
    let mut board = Board::new();
    board.add_city(City::new(city::Owner::Neutral, 400, 500));
    board.add_city(City::new(city::Owner::Neutral, 300, 200));
    board.add_city(City::new(city::Owner::Neutral, 500, 200));
    board.add_city(City::new(city::Owner::Neutral, 200, 300));
    board.add_city(City::new(city::Owner::Neutral, 600, 300));
    board.add_city(City::new(city::Owner::Neutral, 500, 400));
    board.add_city(City::new(city::Owner::Player1("p1".to_string()), 400, 100));
    board.current_player = "p1".to_string();
    for tick in 0..25 {
        board.tick = tick;
        bot.send_board(&board.to_string());
        println!("{board}");
        if let Some(coords) = bot.get_move() {
            println!("coords {:?}", coords);
            board.add_move(
                Owner::Player1("p1".to_string()),
                &format!("{}-{}", coords.0, coords.1),
                &format!("{}-{}", coords.2, coords.3),
            );
        };
        board.tick();
    }
}
