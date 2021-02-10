pub struct Player {
    pub x: u8,
    pub y: u8,
    name: String,
}

pub enum MoveDir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Player {
    pub fn new(x: u8, y: u8, name: &str) -> Player {
        Player {
            x: x,
            y: y,
            name: name.to_string(),
        }
    }

    pub fn move_position(&mut self, dir: MoveDir) {
        match dir {
            MoveDir::UP => self.y -= 1,
            MoveDir::DOWN => self.y += 1,
            MoveDir::LEFT => self.x -= 1,
            MoveDir::RIGHT => self.x += 1,
        }
    }
}
