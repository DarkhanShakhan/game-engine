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
    pub fn step(&mut self, x_step: f64, y_step: f64) {
        self.x += x_step;
        self.y += y_step;
    }
}
