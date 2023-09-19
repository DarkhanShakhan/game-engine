use std::ops::Add;

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position { x, y }
    }
    pub fn distance(&self, rhs: &Position) -> f64 {
        ((self.x - rhs.x).powf(2.0) + (self.y - rhs.y).powf(2.0)).sqrt()
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}
