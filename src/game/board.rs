use std::{collections::HashMap, fmt::Display};

use super::{city::City, movement::Movement, *};

pub struct Board {
    pub city_num: i32,
    pub cities: HashMap<String, City>,
    pub tick: u32,
    pub current_player: String,
    pub move_num: i32,
    pub moves: Vec<Movement>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            city_num: 0,
            cities: HashMap::new(),
            tick: 0,
            current_player: String::new(),
            move_num: 0,
            moves: vec![],
        }
    }
    pub fn add_city(&mut self, city: City) {
        self.cities.insert(city.name.clone(), city);
        self.city_num += 1;
    }

    pub fn add_unit(&mut self) {
        for city in &mut self.cities.values_mut() {
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
    pub fn set_city_owner(&mut self, city_name: &str, owner: Owner) {
        if let Some(c) = self.cities.get_mut(city_name) {
            c.owner = owner;
            c.units = 0;
        }
    }
    pub fn add_move(&mut self, player: Owner, city_from: &str, city_to: &str) {
        if city_from == city_to {
            return;
        }
        if let Some(from) = self.cities.get(city_from) {
            if from.owner != player {
                return;
            }
            if let Some(to) = self.cities.get(city_to) {
                self.moves.push(Movement::new(from, to));
                self.move_num += 1;
                self.cities
                    .entry(city_from.to_string())
                    .and_modify(|c| c.units = 0);
            }
        }
    }
    pub fn tick(&mut self) {
        self.add_unit();
        for mv in &mut self.moves {
            //TODO: remove from movements
            //TODO: modify if the city is captured
            if mv.is_complete() {
                continue;
            }
            mv.tick();
            if mv.is_complete() {
                self.move_num -= 1;
                if let Some(to) = self.cities.get_mut(&mv.to_city) {
                    if to.owner == Owner::Player(mv.from_owner.clone()) {
                        to.units += mv.units;
                    } else {
                        let diff = mv.units - to.units;
                        if diff >= 0 {
                            to.owner = Owner::Player(mv.from_owner.clone())
                        }
                        to.units = diff.abs();
                    }
                }
            }
        }
        self.moves.retain(|e| !e.is_complete());
        self.tick += 1;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("{} {} {}\n", self.city_num, self.current_player, self.tick);
        for city in self.cities.values() {
            out += &city.to_string()
        }
        out += &format!("{}\n", self.move_num);
        for movement in &self.moves {
            if !movement.is_complete() {
                out += &movement.to_string()
            }
        }
        write!(f, "{}", out)
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::new();
        for ix in 1..=6 {
            for jx in 1..=6 {
                board.add_city(City::new(Owner::Neutral, ix * 100, jx * 100));
            }
        }
        board
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;
    #[test]
    fn test_display() {
        let mut board = Board::new();
        let city_1 = City::new(Owner::Neutral, 200, 150);
        let city_2 = City::new(Owner::Player("p1".to_string()), 150, 200);
        board.add_city(city_2);
        board.add_city(city_1);
        board.add_move(Owner::Player("p1".to_string()), "150-200", "200-150");
        board.add_move(Owner::Player("p1".to_string()), "170-200", "200-150");
        board.current_player = "p1".to_string();
        assert_eq!(
            board.to_string(),
            "2 p1 0\np1 10 150-200 150 200 0 0\nNeutral 10 200-150 200 150 0 0\n1\n150-200 200-150 p1 Neutral 4 10\n"
        );
        print!("{}", board);
    }
}
