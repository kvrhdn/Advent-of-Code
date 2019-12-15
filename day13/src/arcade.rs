use crate::sparse_grid::SparseGrid;
use intcode::Computer;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Eq, FromPrimitive, PartialEq)]
#[repr(u8)]
pub enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

pub struct Arcade {
    state: State,
    score: i32,
    computer: Computer,
    screen: SparseGrid<Tile>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum State {
    Idle = 0,
    InProgress,
    AwaitingInput,
    GameOver,
}

#[derive(Clone, Copy, Eq, FromPrimitive, PartialEq)]
#[repr(i8)]
pub enum JoystickPosition {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

impl Arcade {
    pub fn new(game_program: Vec<i64>) -> Self {
        Arcade {
            state: State::Idle,
            score: 0,
            computer: Computer::new(game_program),
            screen: SparseGrid::new(),
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_screen(&self) -> &SparseGrid<Tile> {
        &self.screen
    }

    pub fn set_joystick(&mut self, position: JoystickPosition) {
        self.computer.put_input(position as i64);
    }

    pub fn insert_quarters(&mut self) {
        if self.state != State::Idle {
            panic!("can't insert quarters while arcade is not in Idle");
        }
        self.computer.set(0, 2);
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        self.computer.run()?;

        loop {
            if !self.computer.has_output() {
                break;
            }

            let x = self.computer.get_output()
                .ok_or("Arcade::run: expected x value from computer")?;
            let y = self.computer.get_output()
                .ok_or("Arcade::run: expected y value from computer")?;
            let tile = self.computer.get_output()
                .ok_or("Arcade::run: expected tile value from computer")?;

            if x == -1 && y == 0 {
                self.score = tile as i32;
            } else {
                self.screen.set((x as i32, y as i32).into(), FromPrimitive::from_i64(tile).unwrap());
            }
        }

        self.state = match self.computer.get_state() {
            intcode::State::READY => State::InProgress,
            intcode::State::BLOCKED => State::AwaitingInput,
            intcode::State::HALTED => State::GameOver,
            intcode::State::DEAD => State::GameOver,
        };

        Ok(())
    }
}
