pub mod movement;

use crate::player::Player;
use crate::world::World;

pub trait Action {
    fn perform(&self, player: &mut Player, world: &mut World) -> bool;
    fn can_perform(&self, player: &Player, world: &World) -> bool;
    fn to_string(&self, player: &Player) -> String;
}
