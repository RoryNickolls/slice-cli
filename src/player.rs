use std::fmt;

/// Represents a Player
pub struct Player {
    pub x: u8,
    pub y: u8,
    pub name: String,
    pub health: PlayerHealth,
}

/// Possible move directions
pub enum MoveDir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

/// Implements Display for a MoveDir
impl fmt::Display for MoveDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoveDir::UP => write!(f, "UP"),
            MoveDir::DOWN => write!(f, "DOWN"),
            MoveDir::LEFT => write!(f, "LEFT"),
            MoveDir::RIGHT => write!(f, "RIGHT"),
        }
    }
}

/// Implementation of a Player
impl Player {
    pub fn new(x: u8, y: u8, name: &str) -> Player {
        Player {
            x: x,
            y: y,
            name: name.to_string(),
            health: PlayerHealth::new(100),
        }
    }
}

pub struct PlayerHealth {
    health: u8,
}
impl PlayerHealth {
    fn new(max: u8) -> PlayerHealth {
        PlayerHealth { health: max }
    }

    pub fn damage(&mut self, amount: u8) -> bool {
        if self.health > amount {
            self.health -= amount;
            return false;
        }

        self.health = 0;
        true
    }
}
