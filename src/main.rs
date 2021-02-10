mod player;
mod world;
use player::MoveDir;
use world::World;

fn main() {
    let mut world = World::new("Player1", "Player2", 3);
    println!("Player1: X");
    println!("Player2: O");
    println!("{}", world);
    world.player1.move_position(MoveDir::DOWN);
    println!("{}", world);
    world.player2.move_position(MoveDir::RIGHT);
    println!("{}", world);
}
