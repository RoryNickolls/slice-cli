use crate::player::Player;

use std::fmt;

pub struct World {
    pub player1: Player,
    pub player2: Player,
    size: u8,
}

impl World {
    pub fn new(name1: &str, name2: &str, size: u8) -> World {
        World {
            player1: Player::new(1, 0, name1),
            player2: Player::new(1, 2, name2),
            size: size,
        }
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.size {
            let mut line: String = "|".to_owned();
            for j in 0..self.size {
                if self.player1.y == i && self.player1.x == j {
                    line.push_str("X|");
                } else if self.player2.y == i && self.player2.x == j {
                    line.push_str("O|");
                } else {
                    line.push_str(" |");
                }
            }
            write!(f, "{}\n", line);
        }

        Ok(())
    }
}
