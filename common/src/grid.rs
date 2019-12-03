#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn at(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn step(self, dir: Dir) -> Self {
        match dir {
            Dir::Up    => Self { x: self.x, y: self.y + 1 },
            Dir::Down  => Self { x: self.x, y: self.y - 1 },
            Dir::Right => Self { x: self.x + 1, y: self.y },
            Dir::Left  => Self { x: self.x - 1, y: self.y },
        }
    }
}

pub fn distance(p1: Pos, p2: Pos) -> u32 {
    abs(p1.x - p2.x) + abs(p1.y - p2.y)
}

pub fn distance_origin(p: Pos) -> u32 {
    distance(p, Pos::at(0, 0))
}

fn abs(value: i32) -> u32 {
    if value < 0 {
        -value as u32
    } else {
        value as u32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_distance() {
        let cases = vec![
            (Pos::at(0, 0), Pos::at(2, 3), 5),
            (Pos::at(5, 1), Pos::at(-2, 1), 7),
        ];

        for (p1, p2, expected) in cases {
            assert_eq!(distance(p1, p2), expected);
        }
    }

}