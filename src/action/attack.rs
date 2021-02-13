use crate::action::Action;
use crate::player::Player;
use crate::world::World;

pub enum AttackType {
    THRUST,
    SLASH,
    OVERHEAD,
}

pub struct Attack<'a> {
    pub attack_type: AttackType,
    pub target: &'a Player,
}

impl Action for Attack<'_> {
    fn perform(&self, player: &mut Player, world: &mut World) -> bool {
        true
    }

    fn can_perform(&self, player: &Player, world: &World) -> bool {
        true
    }
    fn to_string(&self, player: &Player) -> String {
        match &self.attack_type {
            AttackType::THRUST => format!("{} thrusts {}", player.name, self.target.name),
            AttackType::SLASH => format!("{} slashes {}", player.name, self.target.name),
            AttackType::OVERHEAD => format!("{} overheads {}", player.name, self.target.name),
        }
    }
}
