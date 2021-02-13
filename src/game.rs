use std::fmt;

use crate::action::Move;
use crate::player::{MoveDir, Player};
use crate::world::World;

/// Represents the Game
/// Keeps track of the World and players
pub struct Game {
    pub world: World,
    pub player1: Player,
    pub player2: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: World::new(3),
            player1: Player::new(1, 0, "Player1"),
            player2: Player::new(1, 2, "Player2"),
        }
    }

    /// Runs the game loop
    pub fn run(&mut self) {
        println!("Player1: X");
        println!("Player2: O");
        println!("{}", self);
        self.player1.perform_action(
            Move {
                move_dir: MoveDir::DOWN,
            },
            &mut self.world,
        );

        println!("{}", self);

        self.player2.perform_action(
            Move {
                move_dir: MoveDir::RIGHT,
            },
            &mut self.world,
        );
        println!("{}", self);
    }
}

/// Prints the current state of the game
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.world.size {
            let mut line: String = "|".to_owned();
            for j in 0..self.world.size {
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
