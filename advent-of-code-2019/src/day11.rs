use crate::grid::*;
use crate::intcode::*;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashMap;

#[aoc(day11, part1)]
fn solve_part1(input: &str) -> usize {
    let mut spacecraft = SpacecraftHull::new();

    let program = load_program(input).unwrap();
    let mut robot = Robot::new(program);

    robot.run_on(&mut spacecraft).unwrap();

    spacecraft.panels_painted()
}

#[aoc(day11, part2)]
fn solve_part2(input: &str) -> String {
    let mut spacecraft = SpacecraftHull::new();
    spacecraft.paint((0, 0).into(), Paint::White);

    let program = load_program(input).unwrap();
    let mut robot = Robot::new(program);

    robot.run_on(&mut spacecraft).unwrap();

    let (min_x, max_x) = spacecraft
        .panels
        .iter()
        .map(|(pos, _)| pos.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = spacecraft
        .panels
        .iter()
        .map(|(pos, _)| pos.y)
        .minmax()
        .into_option()
        .unwrap();

    (min_y..=max_y)
        .rev()
        .flat_map(|y| {
            let mut line = (min_x..=max_x)
                .map(|x| match spacecraft.panels.get(&(x, y).into()) {
                    Some(&Paint::White) => '█',
                    _ => ' ',
                })
                .collect::<Vec<_>>();
            line.push('\n');
            line
        })
        .collect::<String>()
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Paint {
    Black,
    White,
}

struct SpacecraftHull {
    panels: HashMap<Pos, Paint>,
}

impl SpacecraftHull {
    fn new() -> Self {
        SpacecraftHull {
            panels: HashMap::new(),
        }
    }

    fn get(&self, pos: Pos) -> Paint {
        *self.panels.get(&pos).unwrap_or(&Paint::Black)
    }

    fn paint(&mut self, pos: Pos, paint: Paint) {
        self.panels.insert(pos, paint);
    }

    fn panels_painted(&self) -> usize {
        self.panels.iter().count()
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
            spacecraft.paint(self.pos, new_paint);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day11.txt");

        assert_eq!(solve_part1(input), 2373);
        assert_eq!(
            solve_part2(input),
            r#" ███   ██  █  █ ███  █    ███  █  █ █  █   
 █  █ █  █ █ █  █  █ █    █  █ █  █ █ █    
 █  █ █    ██   █  █ █    █  █ █  █ ██     
 ███  █    █ █  ███  █    ███  █  █ █ █    
 █    █  █ █ █  █ █  █    █    █  █ █ █    
 █     ██  █  █ █  █ ████ █     ██  █  █   
"#
        );
    }
}
