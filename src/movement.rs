use std::fmt::Display;

pub struct Movement {
    from: String,
    to: String,
    player_from: String,
    player_to: String,
    ticks_to_finish: i32,
    units: i32,
}

impl Movement {
    pub fn new(
        from: &str,
        to: &str,
        player_from: &str,
        player_to: &str,
        ticks_to_finish: i32,
        units: i32,
    ) -> Self {
        Movement {
            from: from.to_string(),
            to: to.to_string(),
            player_from: player_from.to_string(),
            player_to: player_to.to_string(),
            ticks_to_finish,
            units,
        }
    }
}

impl Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {} {} {} {}",
            self.from, self.to, self.player_from, self.player_to, self.ticks_to_finish, self.units
        )
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    #[test]
    fn test_display() {
        let m = Movement::new("city1", "city2", "player1", "player2", 3, 5);
        assert_eq!(m.to_string(), "city1 city2 player1 player2 3 5\n")
    }
}
