use std::fmt::Display;

use super::position::Position;

pub struct City {
    pub owner: Owner,
    pub units: i32,
    pub name: String,
    pub position: Position,
    a: i32,
    b: i32,
}

impl City {
    pub fn new(owner: Owner, x: i32, y: i32) -> Self {
        City {
            owner,
            units: 10,
            name: x.to_string() + "-" + &y.to_string(),
            position: Position::new(x as f64, y as f64),
            a: 0,
            b: 0,
        }
    }
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {} {} {} {} {}",
            self.owner, self.units, self.name, self.position.x, self.position.y, self.a, self.b
        )
    }
}

#[derive(PartialEq)]
pub enum Owner {
    Player(String),
    Neutral,
}

impl Display for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player(name) => write!(f, "{}", name),
            Self::Neutral => write!(f, "Neutral"),
        }
    }
}
