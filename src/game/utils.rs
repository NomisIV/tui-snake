#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

impl Coord {
    pub fn is_outside(&self, top_left: Coord, bottom_right: Coord) -> bool {
        self.x < top_left.x
            || self.x > bottom_right.x
            || self.y < top_left.y
            || self.y > bottom_right.y
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn is_opposing(&self, d: &Direction) -> bool {
        match (self, d.to_owned()) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) => true,
            (Direction::Right, Direction::Left) => true,
            _ => false,
        }
    }
}

pub trait Move {
    fn r#move(&self, d: &Direction) -> Self;
    fn move_reverse(&self, d: &Direction) -> Self;
}

impl Move for Coord {
    fn r#move(&self, d: &Direction) -> Coord {
        let (rx, ry): (i16, i16) = match d {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        Coord {
            x: self.x + rx,
            y: self.y + ry,
        }
    }

    fn move_reverse(&self, d: &Direction) -> Coord {
        let (rx, ry): (i16, i16) = match d {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (1, 0),
            Direction::Right => (-1, 0),
        };

        Coord {
            x: self.x + rx,
            y: self.y + ry,
        }
    }
}
