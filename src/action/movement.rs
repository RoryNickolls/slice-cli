use crate::action::Action;
use crate::player::{MoveDir, Player};
use crate::world::World;

use std::fmt;

pub struct Move {
    pub move_dir: MoveDir,
    pub player: usize,
}

impl Action for Move {
    fn perform(&self, world: &mut World) {
        let mut player = &mut world.players[self.player];
        match self.move_dir {
            MoveDir::UP => player.y -= 1,
            MoveDir::DOWN => player.y += 1,
            MoveDir::LEFT => player.x -= 1,
            MoveDir::RIGHT => player.x += 1,
        }
    }
    fn can_perform(&self, world: &World) -> bool {
        let player = &world.players[self.player];
        match self.move_dir {
            MoveDir::UP => player.y >= 1,
            MoveDir::DOWN => player.y + 1 < world.size,
            MoveDir::LEFT => player.x >= 1,
            MoveDir::RIGHT => player.x + 1 < world.size,
        }
    }

    fn to_string(&self, world: &World) -> String {
        let player = &world.players[self.player];
        format!("{} moves {}", player.name, self.move_dir)
    }
}
