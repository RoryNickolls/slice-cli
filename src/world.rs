/// Represents the game world
pub struct World {
    pub size: u8,
}

/// Implementation of the World
impl World {
    pub fn new(size: u8) -> World {
        World { size: size }
    }
}
