use std::fmt;

use crate::action::attack::{Attack, AttackType};
use crate::action::movement::Move;
use crate::action::Action;
use crate::player::{MoveDir, Player};
use crate::world::World;

/// Represents the Game
/// Keeps track of the World and players
pub struct Game {
    pub world: World,
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: World::new(3),
        }
    }

    /// Runs the game loop
    pub fn run(&mut self) {
        println!("Player1: X");
        println!("Player2: O");
        println!("{}", self.world);
        self.world.perform_action(Move {
            move_dir: MoveDir::DOWN,
            player: 0,
        });

        println!("{}", self.world);

        self.world.perform_action(Move {
            move_dir: MoveDir::RIGHT,
            player: 1,
        });
        println!("{}", self.world);

        self.world.perform_action(Move {
            move_dir: MoveDir::UP,
            player: 1,
        });
        println!("{}", self.world);
        self.world.perform_action(Attack {
            attack_type: AttackType::OVERHEAD,
            player: 1,
            target: 0,
        });
        println!("{}", self.world);
    }
}
