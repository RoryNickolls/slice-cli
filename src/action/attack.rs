use crate::action::Action;
use crate::world::World;

pub enum AttackType {
    THRUST,
    SLASH,
    OVERHEAD,
}

pub struct Attack {
    pub attack_type: AttackType,
    pub player: usize,
    pub target: usize,
}

impl Action for Attack {
    fn perform(&self, world: &mut World) {}

    fn can_perform(&self, world: &World) -> bool {
        let player = &world.players[self.player];
        let target = &world.players[self.target];

        let diff = (target.x as i16 - player.x as i16) + (target.y as i16 - player.y as i16);
        diff.abs() == 1
    }
    fn to_string(&self, world: &World) -> String {
        let player = &world.players[self.player];
        let target = &world.players[self.target];
        match &self.attack_type {
            AttackType::THRUST => format!("{} thrusts {}", player.name, target.name),
            AttackType::SLASH => format!("{} slashes {}", player.name, target.name),
            AttackType::OVERHEAD => format!("{} overheads {}", player.name, target.name),
        }
    }
}
