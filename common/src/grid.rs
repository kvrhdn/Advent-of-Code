#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
