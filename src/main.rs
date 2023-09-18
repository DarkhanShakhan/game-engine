mod board;
mod bot;
mod city;
mod movement;
use board::Board;
use bot::Bot;

use crate::city::Owner;

fn main() {
    simulate()
}

fn simulate() {
    let mut bot_2 = Bot::new("./bot2/bot");
    let mut bot_1 = Bot::new("./bot1/bot.py");
    let mut board = Board::default();
    board.set_city_owner("400-100", city::Owner::Player("p1".to_string()));
    board.set_city_owner("600-300", Owner::Player("p2".to_string()));

    for _tick in 0..78 {
        // if tick % 2 == 0 {
        board.current_player = "p1".to_string();
        bot_1.send_board(&board.to_string());
        println!("{board}");
        if let Some(coords) = bot_1.get_move() {
            println!("{:?}", coords);
            board.add_move(
                Owner::Player("p1".to_string()),
                &format!("{}-{}", coords.0, coords.1),
                &format!("{}-{}", coords.2, coords.3),
            );
        };
        // } else {
        board.current_player = "p2".to_string();
        bot_2.send_board(&board.to_string());
        // println!("{board}");
        if let Some(coords) = bot_2.get_move() {
            println!("{:?}", coords);
            board.add_move(
                Owner::Player("p2".to_string()),
                &format!("{}-{}", coords.0, coords.1),
                &format!("{}-{}", coords.2, coords.3),
            );
            // };
        }

        board.tick();
    }
}
