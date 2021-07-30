use super::food::Food;
use super::snake::Snake;
use super::utils::*;
use super::Drawable;
use super::{Direction, TickResult};
use std::io::{Error, Write};

pub struct GameState {
    pub snake: Snake,
    pub food: Food,
    pub direction: Direction,
    // pub is_running: bool,
    pub ticks: u32,
    pub win_size: Coord,
}

impl GameState {
    pub fn tick(&mut self) -> TickResult {
        // Move snake
        self.snake.move_step(self.direction);

        // Check if game over
        if self.snake.is_overlapping(self.snake.get_head())
            || self
                .snake
                .get_head()
                .is_outside(Coord { x: 1, y: 1 }, self.win_size)
        {
            return TickResult::GameOver;
        }

        // Check if snake is on food tile
        if self.snake.get_head() == self.food.get_pos() {
            self.snake.eat();

            // Make sure to not relocate food to inside the snake
            while self.food.get_pos() == self.snake.get_head()
                || self.snake.is_overlapping(self.food.get_pos())
            {
                self.food = Food::new(&self.win_size);
            }
        }

        self.ticks += 1;
        TickResult::Ok
    }
}

impl Drawable for GameState {
    fn draw(&mut self, buffer: &mut impl Write) -> Result<(), Error> {
        // Draw board and stuff
        self.snake.draw(buffer)?;
        self.food.draw(buffer)?;
        Ok(())
    }
}
