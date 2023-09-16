use std::fmt::Display;

pub struct City {
    pub owner: Owner,
    pub units: i32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    a: i32,
    b: i32,
}

impl City {
    pub fn new(owner: Owner, x: i32, y: i32) -> Self {
        City {
            owner,
            units: 10,
            name: x.to_string() + "-" + &y.to_string(),
            x,
            y,
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
            self.owner, self.units, self.name, self.x, self.y, self.a, self.b
        )
    }
}

#[cfg(test)]
mod city_tests {
    use super::*;
    #[test]
    fn test_display() {
        assert_eq!(
            City::new(Owner::Neutral, 150, 200).to_string(),
            "Neutral 10 150200 150 200 0 0\n"
        );
        assert_eq!(
            City::new(Owner::Player("pl1".to_owned()), 180, 210).to_string(),
            "pl1 10 180210 180 210 0 0\n"
        );
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

#[cfg(test)]
mod owner_tests {
    use super::*;
    #[test]
    fn test_display() {
        assert_eq!(Owner::Player("Player1".to_owned()).to_string(), "Player1");
        assert_eq!(Owner::Neutral.to_string(), "Neutral");
    }
}
