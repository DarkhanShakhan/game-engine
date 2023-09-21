use std::fmt::Display;

use crate::game::city::City;

use super::position::Position;

#[derive(Clone)]
pub struct Movement {
    pub from_city: String,
    pub from_owner: String,
    pub to_city: String,
    pub to_owner: String,
    pub position: Position,
    step: Position,
    pub ticks_to_finish: i32,
    pub units: i32,
}

impl Movement {
    pub fn new(from: &City, to: &City) -> Self {
        let ticks = ticks_to_finish(&from.position, &to.position);
        let step = Position::new(
            (to.position.x - from.position.x) / ticks as f64,
            (to.position.y - from.position.y) / ticks as f64,
        );

        Movement {
            from_city: from.name.clone(),
            from_owner: from.owner.to_string(),
            to_city: to.name.clone(),
            to_owner: to.owner.to_string(),
            ticks_to_finish: ticks_to_finish(&from.position, &to.position),
            position: from.position + step,
            step,
            units: from.units,
        }
    }
    pub fn tick(&mut self) {
        self.ticks_to_finish -= 1;
        self.position = self.position + self.step;
    }
    pub fn is_complete(&self) -> bool {
        self.ticks_to_finish == 0
    }
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
