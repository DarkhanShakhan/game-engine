use std::fmt::Display;

use crate::city::City;

pub struct Movement {
    pub from_city: String,
    pub from_owner: String,
    pub to_city: String,
    pub to_owner: String,
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
        Movement {
            from_city: from.name.clone(),
            from_owner: from.owner.to_string(),
            to_city: to.name.clone(),
            to_owner: to.owner.to_string(),
            ticks_to_finish: ticks,
            x: from.x as f64,
            x_step: (to.x - from.x) as f64 / ticks as f64,
            y: from.y as f64,
            y_step: (to.y - from.y) as f64 / ticks as f64,
            units: from.units,
        }
    }
    pub fn tick(&mut self) {
        self.ticks_to_finish -= 1;
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

#[cfg(test)]
mod movement_tests {
    use super::*;
    #[test]
    fn test_display() {
        let city_from = City::new(crate::city::Owner::Player("p1".to_string()), 100, 120);
        let city_to = City::new(crate::city::Owner::Neutral, 120, 100);
        let m = Movement::new(&city_from, &city_to);
        assert_eq!(m.to_string(), "100120 120100 p1 Neutral 2 10\n")
    }
}
