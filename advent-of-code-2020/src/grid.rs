#![allow(unused)]

use num_traits::{Num, NumAssignOps, Signed};
use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Pos<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> Pos<T>
where
    T: Num,
{
    pub fn at(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn step(self, dir: Dir) -> Self {
        self.stepn(dir, T::one())
    }

    pub fn stepn(self, dir: Dir, count: T) -> Self {
        match dir {
            Dir::Up => Self {
                x: self.x,
                y: self.y + count,
            },
            Dir::Down => Self {
                x: self.x,
                y: self.y - count,
            },
            Dir::Right => Self {
                x: self.x + count,
                y: self.y,
            },
            Dir::Left => Self {
                x: self.x - count,
                y: self.y,
            },
        }
    }
}

impl<T> Add for Pos<T>
where
    T: Num,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Pos<T>
where
    T: NumAssignOps,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> fmt::Display for Pos<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn distance<T>(p1: &Pos<T>, p2: &Pos<T>) -> T
where
    T: Copy + Signed,
{
    ((p2.x - p1.x).abs() + (p2.y - p1.y).abs())
}

pub fn distance_origin<T>(p: &Pos<T>) -> T
where
    T: Copy + Signed,
{
    distance(p, &Pos::at(T::zero(), T::zero()))
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
