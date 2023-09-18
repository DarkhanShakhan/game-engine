use crate::{board::Board, bot::Bot, city::Owner};

pub struct Game {
    pub board: Board,
    bots: Vec<Bot>,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::default();
        let bot_2 = Bot::new("./bot2/bot", "p2");
        let bot_1 = Bot::new("./bot1/bot.py", "p1");
        board.set_city_owner("400-100", Owner::Player("p1".to_string()));
        board.set_city_owner("600-300", Owner::Player("p2".to_string()));
        Game {
            board,
            bots: vec![bot_1, bot_2],
        }
    }

    pub fn tick(&mut self) {
        for b in &mut self.bots {
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
        }
        self.board.tick();
    }
    pub fn is_finished(&self) -> bool {
        self.board.tick == 50
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
    pub fn on_left(&mut self) {}
    pub fn on_right(&mut self) {}
}

pub struct Stats {
    pub tick: u32,
    pub neutral_count: u32,
    pub player_1_count: u32,
    pub player_2_count: u32,
}
