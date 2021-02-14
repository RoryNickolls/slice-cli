use crate::action::attack::{Attack, AttackType};
use crate::action::movement::Move;
use crate::action::Action;
use crate::player::MoveDir;
use crate::world::World;

use std::io::{stdin, stdout, Read, Write};
use termion::color;
use termion::event::Key;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

/// Represents the Game
/// Keeps track of the World and players
pub struct Game {
    pub world: World,
    turn: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: World::new(3),
            turn: 0,
        }
    }

    /// Runs the game loop
    pub fn run(&mut self) {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        println!(
            "{}{}Welcome to {}SLICE CLI{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::color::Fg(termion::color::Red),
            termion::color::Fg(termion::color::Reset),
        );

        let mut action: Option<Box<dyn Action>> = None;
        loop {
            println!(
                "{}{}",
                termion::cursor::Goto(1, 2),
                termion::clear::AfterCursor,
            );
            if let Some(a) = action {
                println!("{}", a.to_string(&self.world));
            }
            println!("{}", self.world);
            println!(
                "{}Player {}{} to go.",
                termion::color::Fg(termion::color::Yellow),
                self.turn + 1,
                termion::color::Fg(termion::color::Reset)
            );
            println!("1 Move\t2 Attack");
            stdout.flush().unwrap();

            let stdin = stdin();
            let stdin = stdin.lock();
            let mut bytes = stdin.bytes();
            loop {
                let b = bytes.next().unwrap().unwrap();
                match b {
                    b'q' => return,
                    b'1' => {
                        action = Some(Box::new(Move {
                            move_dir: MoveDir::DOWN,
                            player: self.turn as usize,
                        }));
                        break;
                    }
                    b'2' => println!("attacking"),
                    _ => (),
                }
            }
            self.turn = (self.turn + 1) % 2;
        }
    }
}
