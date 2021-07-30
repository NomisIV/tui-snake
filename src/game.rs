mod food;
mod game_state;
mod snake;
pub mod utils;

use std::io::{Error, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use food::Food;
use game_state::GameState;
use snake::Snake;
use utils::*;

const TICK_RATE: u64 = 100;
const INITIAL_SNAKE_LENGTH: u16 = 3;

pub enum GameCommand {
    Tick,
    Move(Direction),
    Stop,
}

pub enum TickResult {
    Ok,
    GameOver,
}

pub trait Drawable {
    fn draw(&mut self, buffer: &mut impl Write) -> Result<(), Error>;
}

pub struct Game {
    transmitter: Sender<GameCommand>,
    reciever: Receiver<GameCommand>,
    game_state: GameState,
}

impl Game {
    pub fn new(win_size: &Coord) -> (Game, Sender<GameCommand>) {
        let (tx, rx): (Sender<GameCommand>, Receiver<GameCommand>) = mpsc::channel();

        let game_size = Coord {
            x: win_size.x / 2,
            y: win_size.y,
        };

        let game_state = GameState {
            snake: Snake::new(Coord { x: 5, y: 5 }, INITIAL_SNAKE_LENGTH),
            food: Food::new(&game_size),
            direction: Direction::Right,
            ticks: 0,
            win_size: game_size,
        };

        let game = Game {
            transmitter: tx.clone(),
            reciever: rx,
            game_state,
        };

        (game, tx)
    }

    pub fn start(&mut self, buffer: &mut impl Write) -> Result<(), Error> {
        let tx = self.transmitter.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(TICK_RATE));
            tx.send(GameCommand::Tick).unwrap();
        });

        write!(buffer, "{}", termion::cursor::Hide,)?;

        // This is the "main process"
        loop {
            match self.reciever.recv().unwrap() {
                GameCommand::Tick => match self.game_state.tick() {
                    TickResult::GameOver => break,
                    TickResult::Ok => {
                        write!(buffer, "{}", termion::clear::All)?;
                        self.game_state.draw(buffer)?;
                        buffer.flush()?;
                    }
                },
                GameCommand::Move(direction) => {
                    if !self.game_state.direction.is_opposing(&direction) {
                        self.game_state.direction = direction;
                    }
                }
                GameCommand::Stop => break,
            }
        }

        write!(
            buffer,
            "{}{}{}Game Over!\n\rFinal Score: {}\n\r",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show,
            self.game_state.snake.get_length() - INITIAL_SNAKE_LENGTH
        )?;

        Ok(())
    }
}
