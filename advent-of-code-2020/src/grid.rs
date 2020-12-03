#![allow(unused)]

use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Pos {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Pos {
    pub fn at(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn step(self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Self {
                x: self.x,
                y: self.y + 1,
            },
            Dir::Down => Self {
                x: self.x,
                y: self.y - 1,
            },
            Dir::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Dir::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn distance(p1: &Pos, p2: &Pos) -> u32 {
    ((p2.x - p1.x).abs() + (p2.y - p1.y).abs()) as u32
}

pub fn distance_origin(p: &Pos) -> u32 {
    distance(p, &Pos::at(0, 0))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn turn_left(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_add() {
        assert_eq!(Pos::at(2, 1) + Pos::at(3, 4), Pos::at(5, 5));
        assert_eq!(Pos::at(-2, 1) + Pos::at(3, -4), Pos::at(1, -3));
    }

    #[test]
    fn test_distance() {
        let cases = vec![
            (Pos::at(2, 3), Pos::at(2, 3), 0),
            (Pos::at(0, 0), Pos::at(2, 3), 5),
            (Pos::at(5, 1), Pos::at(-2, 1), 7),
        ];

        for (p1, p2, expected) in cases {
            assert_eq!(distance(&p1, &p2), expected);
        }
    }

    #[test]
    fn test_distance_origin() {
        let cases = vec![(Pos::at(0, 0), 0), (Pos::at(5, 1), 6)];

        for (p, expected) in cases {
            assert_eq!(distance_origin(&p), expected);
        }
    }
}
