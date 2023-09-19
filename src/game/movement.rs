use std::fmt::Display;

use crate::game::city::City;

use super::position::Position;

pub struct Movement {
    pub from_city: String,
    pub from_owner: String,
    pub to_city: String,
    pub to_owner: String,
    pub position: Position,
    step: Position,
    pub x: f64,
    x_step: f64,
    y_step: f64,
    pub y: f64,
    ticks_to_finish: i32,
    pub units: i32,
}

impl Movement {
    pub fn new(from: &City, to: &City) -> Self {
        let ticks = ticks((from.x, from.y), (to.x, to.y));
        let x_step = (to.x - from.x) as f64 / ticks as f64;
        let y_step = (to.y - from.y) as f64 / ticks as f64;

        Movement {
            from_city: from.name.clone(),
            from_owner: from.owner.to_string(),
            to_city: to.name.clone(),
            to_owner: to.owner.to_string(),
            ticks_to_finish: ticks_to_finish(&from.position, &to.position),
            position: Position::new(from.x as f64 + x_step, from.y as f64 + y_step),
            step: Position::new(x_step, y_step),
            x: from.x as f64 + x_step,
            x_step,
            y: from.y as f64 + y_step,
            y_step,
            units: from.units,
        }
    }
    pub fn tick(&mut self) {
        self.ticks_to_finish -= 1;
        self.position = self.position + self.step;
        self.x += self.x_step;
        self.y += self.y_step;
    }
    pub fn is_complete(&self) -> bool {
        self.ticks_to_finish == 0
    }
}

fn ticks(from: (i32, i32), to: (i32, i32)) -> i32 {
    let distance = (((from.0 - to.0).pow(2) + (from.1 - to.1).pow(2)) as f32).sqrt();
    (distance / 50.0).ceil() as i32
}

fn ticks_to_finish(from: &Position, to: &Position) -> i32 {
    (from.distance(to) / 50.0).ceil() as i32
}

impl Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {} {} {} {}",
            self.from_city,
            self.to_city,
            self.from_owner,
            self.to_owner,
            self.ticks_to_finish,
            self.units
        )
    }
}
