use std::fmt::Display;

use crate::{
    city::{City, Owner},
    movement::Movement,
};

pub struct Board {
    pub city_num: i32,
    pub cities: Vec<City>,
    pub tick: u32,
    pub current_player: String,
    pub move_num: i32,
    pub moves: Vec<Movement>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            city_num: 0,
            cities: vec![],
            tick: 0,
            current_player: String::new(),
            move_num: 0,
            moves: vec![],
        }
    }
    pub fn add_city(&mut self, city: City) {
        self.cities.push(city);
        self.city_num += 1;
    }

    pub fn add_unit(&mut self) {
        for city in &mut self.cities {
            match city.owner {
                Owner::Neutral => {
                    if city.units < 10 {
                        city.units += 1;
                    }
                }
                _ => {
                    if city.units < 50 {
                        city.units += 1
                    }
                }
            }
        }
    }

    pub fn add_move(&mut self, movement: Movement) {
        self.moves.push(movement);
        self.move_num += 1;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("{} {} {}\n", self.city_num, self.current_player, self.tick);
        for city in &self.cities {
            out += &city.to_string()
        }
        out += &format!("{}\n", self.move_num);
        for movement in &self.moves {
            out += &movement.to_string()
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
        board.add_city(City::new(crate::city::Owner::Neutral, 200, 150));
        board.add_move(Movement::new(
            "200150", "100510", "player_1", "neutral", 5, 15,
        ));
        board.current_player = "player_1".to_string();
        assert_eq!(
            board.to_string(),
            "1 player_1 0\nNeutral 10 200150 200 150 0 0\n1\n200150 100510 player_1 neutral 5 15\n"
        );
        print!("{}", board);
    }
}
