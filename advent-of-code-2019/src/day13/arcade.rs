use crate::intcode::Computer;
use crate::{grid::Pos, intcode};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;

#[derive(Eq, FromPrimitive, PartialEq)]
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

pub enum JoystickPosition {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum State {
    Idle,
    InProgress,
    AwaitingInput,
    GameOver,
}

pub struct Arcade {
    state: State,
    score: i64,
    computer: Computer,
    screen: HashMap<Pos, Tile>,
}

impl Arcade {
    pub fn new(game_program: Vec<i64>) -> Self {
        Arcade {
            state: State::Idle,
            score: 0,
            computer: Computer::new(game_program),
            screen: HashMap::new(),
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn get_score(&self) -> i64 {
        self.score
    }

    pub fn tiles(&self, tile: Tile) -> impl Iterator<Item = &Pos> {
        self.screen
            .iter()
            .filter(move |&(_, value)| *value == tile)
            .map(|(pos, _)| pos)
    }

    pub fn set_joystick(&mut self, position: JoystickPosition) {
        self.computer.put_input(position as i64);
    }

    pub fn insert_quarter(&mut self) -> Result<(), &'static str> {
        if self.state != State::Idle {
            return Err("can't insert quarters while arcade is not in Idle");
        }
        self.computer.set(0, 2);
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        self.computer.run()?;

        while self.computer.has_output() {
            let x = self
                .computer
                .get_output()
                .ok_or("Arcade::run: expected x value from computer")?;
            let y = self
                .computer
                .get_output()
                .ok_or("Arcade::run: expected y value from computer")?;
            let tile = self
                .computer
                .get_output()
                .ok_or("Arcade::run: expected tile value from computer")?;

            if x == -1 && y == 0 {
                self.score = tile;
            } else {
                self.screen.insert(
                    Pos::at(x as i32, y as i32),
                    FromPrimitive::from_i64(tile).unwrap(),
                );
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
