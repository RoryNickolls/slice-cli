use crate::action::Action;
use crate::player::{MoveDir, Player};
use crate::world::World;

use std::fmt;

pub struct Move {
    pub move_dir: MoveDir,
}

impl Action for Move {
    fn perform(&self, player: &mut Player, world: &mut World) -> bool {
        if self.can_perform(player, world) {
            match self.move_dir {
                MoveDir::UP => player.y -= 1,
                MoveDir::DOWN => player.y += 1,
                MoveDir::LEFT => player.x -= 1,
                MoveDir::RIGHT => player.x += 1,
            }

            return true;
        }
        false
    }

    fn can_perform(&self, player: &Player, world: &World) -> bool {
        true
    }

    fn to_string(&self, player: &Player) -> String {
        format!("{} moves {}", player.name, self.move_dir)
    }
}
