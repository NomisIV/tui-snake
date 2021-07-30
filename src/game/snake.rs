use crate::game::utils::*;
use crate::game::Drawable;

use std::io::{Error, Write};

pub struct Snake {
    head: Coord,
    body: Vec<Direction>,
    max_length: u16,
}

impl Snake {
    pub fn new(start_pos: Coord, len: u16) -> Snake {
        let head = start_pos;
        let body = vec![Direction::Right; len.into()];
        let max_length = len;

        Snake {
            head,
            body,
            max_length,
        }
    }

    pub fn move_step(&mut self, d: Direction) {
        self.head = self.head.r#move(&d);
        self.body.insert(0, d);
        if self.body.len() > self.max_length.into() {
            self.body.pop();
        }
    }

    pub fn eat(&mut self) {
        self.max_length += 1;
    }

    pub fn is_overlapping(&self, c: Coord) -> bool {
        let mut pos = self.head.clone();
        self.body
            .iter()
            .filter(|s| {
                pos = pos.r#move(s.to_owned());
                c == pos
            })
            .count()
            .gt(&0)
    }

    pub fn get_head(&self) -> Coord {
        self.head.clone()
    }

    pub fn get_length(&self) -> u16 {
        self.max_length
    }
}

impl Drawable for Snake {
    fn draw(&mut self, buffer: &mut impl Write) -> Result<(), Error> {
        let mut pos = self.head;
        write!(
            buffer,
            "{}{}[]{}",
            termion::cursor::Goto((pos.x * 2) as u16, pos.y as u16),
            termion::style::Bold,
            termion::style::Reset,
        )?;
        for n in 0..self.body.len() {
            pos = pos.move_reverse(self.body.get(n as usize).unwrap());
            write!(
                buffer,
                "{}{}[]{}",
                termion::cursor::Goto((pos.x * 2) as u16, pos.y as u16),
                termion::style::Bold,
                termion::style::Reset,
            )?;
        }

        Ok(())
    }
}
