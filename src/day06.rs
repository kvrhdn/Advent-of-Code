use aoc_runner_derive::{aoc, aoc_generator};
use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Eq, PartialEq)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "turn on" => Ok(Command::TurnOn),
            "turn off" => Ok(Command::TurnOff),
            "toggle" => Ok(Command::Toggle),
            _ => Err(format!("Unrecognized command: {}", s)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    command: Command,
    from: Pos,
    to: Pos,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        }
        let captures = RE.captures(s).unwrap().unwrap();

        assert!(captures.len() == 6);

        let command = captures.get(1).unwrap().as_str().try_into().unwrap();
        let from = Pos {
            x: captures.get(2).unwrap().as_str().parse().unwrap(),
            y: captures.get(3).unwrap().as_str().parse().unwrap(),
        };
        let to = Pos {
            x: captures.get(4).unwrap().as_str().parse().unwrap(),
            y: captures.get(5).unwrap().as_str().parse().unwrap(),
        };

        Instruction { command, from, to }
    }
}

#[aoc_generator(day6)]
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| Instruction::parse(l)).collect()
}

#[aoc(day6, part1)]
fn solve_part1(instructions: &[Instruction]) -> u32 {
    let mut lights = vec![false; 1000 * 1000];

    for ir in instructions {
        for y in ir.from.y..=ir.to.y {
            for x in ir.from.x..=ir.to.x {
                let entry = lights.get_mut((1000 * y) + x).unwrap();
                *entry = match ir.command {
                    Command::TurnOn => true,
                    Command::TurnOff => false,
                    Command::Toggle => !*entry,
                }
            }
        }
    }

    lights.iter().filter(|&&l| l).count() as u32
}

#[aoc(day6, part2)]
fn solve_part2(instructions: &[Instruction]) -> u32 {
    let mut lights = vec![0u32; 1000 * 1000];

    for ir in instructions {
        for y in ir.from.y..=ir.to.y {
            for x in ir.from.x..=ir.to.x {
                let entry = lights.get_mut((1000 * y) + x).unwrap();
                match ir.command {
                    Command::TurnOn => {
                        *entry += 1;
                    }
                    Command::TurnOff => {
                        *entry = entry.saturating_sub(1);
                    }
                    Command::Toggle => {
                        *entry += 2;
                    }
                }
            }
        }
    }

    lights.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_parse() {
        let examples = vec![
            (
                "turn on 0,0 through 999,999",
                Instruction {
                    command: Command::TurnOn,
                    from: Pos { x: 0, y: 0 },
                    to: Pos { x: 999, y: 999 },
                },
            ),
            (
                "toggle 0,0 through 999,0",
                Instruction {
                    command: Command::Toggle,
                    from: Pos { x: 0, y: 0 },
                    to: Pos { x: 999, y: 0 },
                },
            ),
            (
                "turn off 499,499 through 500,500",
                Instruction {
                    command: Command::TurnOff,
                    from: Pos { x: 499, y: 499 },
                    to: Pos { x: 500, y: 500 },
                },
            ),
        ];

        for (input, expected) in examples {
            assert_eq!(Instruction::parse(input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day6.txt");

        let instructions = parse_instructions(input);

        assert_eq!(solve_part1(&instructions), 569999);
        assert_eq!(solve_part2(&instructions), 17836115);
    }
}
