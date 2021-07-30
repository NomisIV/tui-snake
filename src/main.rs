mod game;

use std::io::{stdin, stdout};
use std::thread;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use game::utils::*;
use game::*;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (win_x, win_y) = termion::terminal_size().unwrap();

    // Create game object
    let (mut game, tx) = Game::new(&Coord {
        x: win_x as i16,
        y: win_y as i16,
    });

    // Listen for inputs
    thread::spawn(move || {
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('c') | Key::Esc => tx.send(GameCommand::Stop).unwrap(),
                Key::Up => tx.send(GameCommand::Move(Direction::Up)).unwrap(),
                Key::Down => tx.send(GameCommand::Move(Direction::Down)).unwrap(),
                Key::Left => tx.send(GameCommand::Move(Direction::Left)).unwrap(),
                Key::Right => tx.send(GameCommand::Move(Direction::Right)).unwrap(),
                _ => {}
            }
        }
    });

    game.start(&mut stdout).expect("Error");
}
