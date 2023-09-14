use std::fmt::Display;

use crate::city::City;

pub struct Board {
    pub city_num: i32,
    pub cities: Vec<City>,
    pub tick: u32,
    pub current_player: String,
}

impl Board {
    pub fn new() -> Self {
        Board {
            city_num: 0,
            cities: vec![],
            tick: 0,
            current_player: String::new(),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("{} {} {}\n", self.city_num, self.current_player, self.tick);
        for city in &self.cities {
            out += &city.to_string()
        }
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;
    #[test]
    fn test_display() {
        let mut board = Board::new();
        board
            .cities
            .push(City::new(crate::city::Owner::Neutral, 200, 150));
        board.current_player = "player_1".to_string();
        assert_eq!(
            board.to_string(),
            "0 player_1 0\nNeutral 10 200150 200 150 0 0\n"
        );
        print!("{}", board);
    }
}
