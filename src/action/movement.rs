use crate::action::Action;
use crate::player::MoveDir;
use crate::world::World;

pub struct Move {
    pub move_dir: MoveDir,
    pub player: usize,
}

impl Action for Move {
    /// Performs a movement
    fn perform(&self, world: &mut World) {
        let mut player = &mut world.players[self.player];
        match self.move_dir {
            MoveDir::UP => player.y -= 1,
            MoveDir::DOWN => player.y += 1,
            MoveDir::LEFT => player.x -= 1,
            MoveDir::RIGHT => player.x += 1,
        }
    }

    /// Checks that the new position is free and in bounds
    fn can_perform(&self, world: &World) -> bool {
        let player = &world.players[self.player];
        let mut new_x = player.x as i16;
        let mut new_y = player.y as i16;
        match self.move_dir {
            MoveDir::UP => new_y -= 1,
            MoveDir::DOWN => new_y += 1,
            MoveDir::LEFT => new_x -= 1,
            MoveDir::RIGHT => new_x += 1,
        };

        return world.is_free(new_x as u8, new_y as u8)
            && new_x >= 0
            && new_y >= 0
            && new_x < world.size as i16
            && new_y < world.size as i16;
    }

    fn to_string(&self, world: &World) -> String {
        let player = &world.players[self.player];
        format!("{} moves {}", player.name, self.move_dir)
    }
}
