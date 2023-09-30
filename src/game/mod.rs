use std::collections::HashMap;

use self::{board::Board, bot::Bot, city::Owner};

pub mod board;
pub mod bot;
pub mod city;
pub mod movement;
pub mod position;

pub struct Game {
    board_hitory: HashMap<u32, Board>,
    pub board: Board,
    pub bots: Vec<Bot>,
}

impl Game {
    pub fn new(bot_1_path: &str, bot_2_path: &str) -> Self {
        let mut board = Board::default();
        let bot_1 = Bot::new(bot_1_path, "p1");
        let bot_2 = Bot::new(bot_2_path, "p2");
        board.set_city_owner("100-100", Owner::Player("p1".to_string()));
        board.set_city_owner("600-600", Owner::Player("p2".to_string()));
        Game {
            board_hitory: HashMap::new(),
            board,
            bots: vec![bot_1, bot_2],
        }
    }

    pub fn tick(&mut self) {
        if let Some(board) = self.board_hitory.get(&(self.board.tick + 1)) {
            self.board = board.clone();
            return;
        }
        for (ix, b) in self.bots.iter_mut().enumerate() {
            self.board.current_player = b.name.clone();
            if b.send_board(&self.board.to_string()).is_err() {
                continue;
            };
            if let Some(coords) = b.get_move() {
                self.board.add_move(
                    Owner::Player(self.board.current_player.clone()),
                    &format!("{}-{}", coords.0, coords.1),
                    &format!("{}-{}", coords.2, coords.3),
                )
            }
            if let Some(log) = b.get_debug() {
                if ix == 0 {
                    self.board.p1_logs = log;
                } else {
                    self.board.p2_logs = log;
                }
            }
        }
        self.board.tick();
        self.board_hitory
            .insert(self.board.tick, self.board.clone());
    }
    pub fn is_finished(&self) -> bool {
        self.board.tick == 300
    }
    pub fn stats(&self) -> Stats {
        let mut stats = Stats {
            tick: self.board.tick,
            neutral_count: 0,
            player_1_count: 0,
            player_2_count: 0,
        };
        for c in self.board.cities.values() {
            if let Owner::Player(name) = &c.owner {
                if name == "p1" {
                    stats.player_1_count += 1;
                } else {
                    stats.player_2_count += 1;
                }
            } else {
                stats.neutral_count += 1;
            }
        }
        stats
    }
    pub fn on_left(&mut self) {
        if self.board.tick != 0 {
            if let Some(board) = self.board_hitory.get(&(self.board.tick - 1)) {
                self.board = board.clone();
            }
        }
    }
    //FIXME:cpu intensive => store snapshot of state, not the board itself
    pub fn on_right(&mut self) {
        if !self.is_finished() {
            if let Some(board) = self.board_hitory.get(&(self.board.tick + 1)) {
                self.board = board.clone();
            }
        }
    }
}

pub struct Stats {
    pub tick: u32,
    pub neutral_count: u32,
    pub player_1_count: u32,
    pub player_2_count: u32,
}
