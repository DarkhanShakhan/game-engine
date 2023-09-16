mod board;
mod bot;
mod city;
mod movement;

use board::Board;
use bot::Bot;
use city::City;

fn main() {
    let mut bot = Bot::new("./bot1/bot.py");
    let mut board = Board::new();
    board.add_city(City::new(city::Owner::Neutral, 100, 250));
    board.add_city(City::new(city::Owner::Player1("p1".to_string()), 100, 750));
    board.current_player = "p1".to_string();
    for tick in 0..3 {
        board.tick = tick;
        bot.send_board(&board.to_string());
        println!("{board}");
        let coords = bot.get_move();
        println!("coords {:?}", coords);
        board.add_unit();
    }
}
