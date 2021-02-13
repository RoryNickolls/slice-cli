mod action;
mod game;
mod player;
mod world;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
