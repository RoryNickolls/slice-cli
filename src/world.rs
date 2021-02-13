use crate::action::Action;
use crate::player::Player;

use std::fmt;

/// Represents the game world
pub struct World {
    pub size: u8,
    pub players: [Player; 2],
}

/// Implementation of the World
impl World {
    pub fn new(size: u8) -> World {
        World {
            size: size,
            players: [Player::new(1, 0, "Player1"), Player::new(1, 2, "Player2")],
        }
    }

    /// Performs an Action on the World
    pub fn perform_action(&mut self, action: impl Action) {
        if action.can_perform(self) {
            println!("{}", action.to_string(self));
            action.perform(self);
        } else {
            println!("Failed to perform action.");
        }
    }

    /// Checks whether a position is free to move to
    pub fn is_free(&self, x: u8, y: u8) -> bool {
        (x != self.players[0].x || y != self.players[0].y)
            && (x != self.players[1].x || y != self.players[1].y)
    }
}

/// Prints the current World state
impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.size {
            let mut line: String = "|".to_owned();
            for j in 0..self.size {
                if self.players[0].y == i && self.players[0].x == j {
                    line.push_str("X|");
                } else if self.players[1].y == i && self.players[1].x == j {
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
