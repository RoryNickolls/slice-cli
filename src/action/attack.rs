use crate::action::Action;
use crate::player::Player;

use std::fmt;

pub enum AttackType {
    THRUST,
    SLASH,
    OVERHEAD,
}

pub struct Attack {
    pub attack_type: AttackType,
    pub attacker: &Player,
    pub target: &Player,
}

impl Action for Attack {
    fn perform(&self, player: &mut Player, world: &mut World) {}

    fn can_perform(&self, player: &Player, world: &World) {
        true
    }
    fn to_string(&self, player: &Player) -> String {
        match self.attack_type {
            THRUST => format!("{} thrusts {}", player.name, self.target.name),
            SLASH => format!("{} slashes {}", player.name, self.target.name),
            OVERHEAD => format!("{} overheads {}", player.name, self.target.name),
        }
    }
}
