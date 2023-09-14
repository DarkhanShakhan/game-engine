mod board;
mod bot;
mod city;

use bot::Bot;

fn main() {
    let mut bot1 = Bot::new("./bot1/bot.py");
    let mut bot2 = Bot::new("./bot2/bot.py");

    let mut board = [[""; 3]; 3];

    for turn in 0..9 {
        let current_bot = if turn % 2 == 0 { &mut bot1 } else { &mut bot2 };
        let symbol = if turn % 2 == 0 { "X" } else { "O" };

        let board_ref: [[&str; 3]; 3] = [
            [&board[0][0], &board[0][1], &board[0][2]],
            [&board[1][0], &board[1][1], &board[1][2]],
            [&board[2][0], &board[2][1], &board[2][2]],
        ];

        current_bot.send_board(&board_ref);

        let (x, y) = current_bot.get_move();

        if board[x][y] == "" {
            board[x][y] = symbol;
        } else {
            println!("Invalid move by {} at {},{}", symbol, x, y);
            break;
        }

        // Add logic here to check for a winner, if necessary
    }
}
