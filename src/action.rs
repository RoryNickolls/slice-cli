pub mod attack;
pub mod movement;

use crate::world::World;

pub trait Action {
    fn perform(&self, world: &mut World);
    fn can_perform(&self, world: &World) -> bool;
    fn to_string(&self, world: &World) -> String;
}
