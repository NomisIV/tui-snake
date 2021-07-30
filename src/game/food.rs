use crate::game::utils::Coord;
use crate::game::Drawable;

use std::io::{Error, Write};

use rand::{thread_rng, Rng};

pub struct Food {
    pos: Coord,
}

impl Food {
    pub fn new(win_size: &Coord) -> Food {
        let mut rng = thread_rng();
        let (x, y) = (rng.gen_range(1..win_size.x), rng.gen_range(1..win_size.y));
        Food {
            pos: Coord { x, y },
        }
    }

    pub fn get_pos(&self) -> Coord {
        self.pos.clone()
    }
}

impl Drawable for Food {
    fn draw(&mut self, buffer: &mut impl Write) -> Result<(), Error> {
        write!(
            buffer,
            "{}{}{}(){}{}",
            termion::cursor::Goto((self.pos.x * 2) as u16, self.pos.y as u16),
            termion::style::Bold,
            termion::color::Fg(termion::color::LightRed),
            termion::style::Reset,
            termion::color::Fg(termion::color::Reset),
        )?;
        Ok(())
    }
}
