use common::console_utils::Timer;
use common::grid::*;
use intcode::*;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/11
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part1");

    let mut spacecraft = SpacecraftHull::new();

    let program = load_program(input)?;
    let mut robot = Robot::new(program);

    robot.run_on(&mut spacecraft)?;

    Ok(spacecraft.panels_once_painted.len() as i32)
}

/// See: https://adventofcode.com/2019/day/11#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<JsValue, JsValue> {
    Timer::new("rust::part2");

    let mut spacecraft = SpacecraftHull::new();
    spacecraft.set((0, 0).into(), Paint::White);

    let program = load_program(input)?;
    let mut robot = Robot::new(program);

    robot.run_on(&mut spacecraft)?;

    // get min/max of all painted cells
    let min_x = spacecraft.panels.iter().map(|(pos, _)| pos.x).min().unwrap_or(0);
    let max_x = spacecraft.panels.iter().map(|(pos, _)| pos.x).max().unwrap_or(0);
    let min_y = spacecraft.panels.iter().map(|(pos, _)| pos.y).min().unwrap_or(0);
    let max_y = spacecraft.panels.iter().map(|(pos, _)| pos.y).max().unwrap_or(0);

    let mut image = Vec::<char>::new();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let panel = match spacecraft.panels.get(&(x, y).into()) {
                Some(&Paint::White) => '█',
                _ => ' ',
            };
            image.push(panel);
        }
        image.push('\n');
    }

    let result = image.into_iter().collect::<String>();

    console::log_1(&"The solution to day 11, part 2:".into());
    console::log_1(&result.into());

    Ok("See the console".into())
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Paint {
    Black = 0,
    White = 1,
}

struct SpacecraftHull {
    panels: HashMap<Pos, Paint>,
    panels_once_painted: HashSet<Pos>,
}

impl SpacecraftHull {
    fn new() -> Self {
        SpacecraftHull {
            panels: HashMap::new(),
            panels_once_painted: HashSet::new(),
        }
    }

    fn get(&self, pos: Pos) -> Paint {
        *self.panels.get(&pos).unwrap_or(&Paint::Black)
    }

    fn set(&mut self, pos: Pos, paint: Paint) {
        self.panels.insert(pos, paint);

        if paint == Paint::White {
            self.panels_once_painted.insert(pos);
        }
    }
}

struct Robot {
    pos: Pos,
    dir: Dir,
    computer: Computer,
}

impl Robot {
    fn new(program: Vec<i64>) -> Self {
        Self {
            pos: Pos::at(0, 0),
            dir: Dir::Up,
            computer: Computer::new(program),
        }
    }

    fn run_on(&mut self, spacecraft: &mut SpacecraftHull) -> Result<(), &'static str> {
        loop {
            let input = spacecraft.get(self.pos);

            self.computer.put_input(input as i64);

            self.computer.run()?;

            if self.computer.get_state() == State::HALTED {
                return Ok(());
            }

            let new_paint = match self.computer.get_output() {
                Some(0) => Paint::Black,
                Some(1) => Paint::White,
                Some(_) => return Err("program output an unexpected value"),
                None => return Err("program did not output any value"),
            };
            spacecraft.set(self.pos, new_paint);

            self.dir = match self.computer.get_output() {
                Some(0) => self.dir.turn_left(),
                Some(1) => self.dir.turn_right(),
                Some(_) => return Err("program output an unexpected value"),
                None => return Err("program did not output any value"),
            };

            self.pos = self.pos.step(self.dir);
        }
    }
}
