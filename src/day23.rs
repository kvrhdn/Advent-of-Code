use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Register {
    A,
    B,
}

impl TryFrom<&str> for Register {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            v => Err(format!("unknown register {}", v)),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Offset(isize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset),
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<op>\w{3}) ?(?P<reg>[ab])?,? ?(?P<offset>[+-]\d+)?$").unwrap();
        }
        let captures = RE.captures(input).unwrap();

        let op = captures.name("op").unwrap().as_str();
        let get_register = || captures.name("reg").unwrap().as_str().try_into().unwrap();
        let get_offset = || Offset(captures.name("offset").unwrap().as_str().parse().unwrap());

        match op {
            "hlf" => Instruction::Half(get_register()),
            "tpl" => Instruction::Triple(get_register()),
            "inc" => Instruction::Increment(get_register()),
            "jmp" => Instruction::Jump(get_offset()),
            "jie" => Instruction::JumpIfEven(get_register(), get_offset()),
            "jio" => Instruction::JumpIfOne(get_register(), get_offset()),
            v => panic!("uknown op {}", v),
        }
    }
}

struct Computer {
    a: u32,
    b: u32,
    pc: isize,
}

impl Computer {
    fn new() -> Self {
        Self { a: 0, b: 0, pc: 0 }
    }

    fn get(&self, r: Register) -> u32 {
        match r {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    fn set(&mut self, r: Register, value: u32) {
        match r {
            Register::A => self.a = value,
            Register::B => self.b = value,
        }
    }

    fn modify(&mut self, r: Register, f: fn(u32) -> u32) {
        match r {
            Register::A => self.a = f(self.a),
            Register::B => self.b = f(self.b),
        }
    }

    fn execute(&mut self, program: &[Instruction]) {
        loop {
            let ir = match program.get(self.pc as usize) {
                Some(ir) => ir,
                None => return,
            };

            match ir {
                Instruction::Half(r) => {
                    self.modify(*r, |v| v / 2);
                }
                Instruction::Triple(r) => {
                    self.modify(*r, |v| v * 3);
                }
                Instruction::Increment(r) => {
                    self.modify(*r, |v| v + 1);
                }
                Instruction::Jump(offset) => {
                    self.pc += offset.0 - 1;
                }
                Instruction::JumpIfEven(r, offset) => {
                    if self.get(*r) % 2 == 0 {
                        self.pc += offset.0 - 1;
                    }
                }
                Instruction::JumpIfOne(r, offset) => {
                    if self.get(*r) == 1 {
                        self.pc += offset.0 - 1;
                    }
                }
            }

            self.pc += 1;
        }
    }
}

#[aoc_generator(day23, part1)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| Instruction::parse(l)).collect()
}

#[aoc_generator(day23, part2)]
fn parse_input_part2(input: &str) -> Vec<Instruction> {
    parse_input(input)
}

#[aoc(day23, part1)]
fn solve_part1(program: &[Instruction]) -> u32 {
    let mut computer = Computer::new();

    computer.execute(&program);

    computer.get(Register::B)
}

#[aoc(day23, part2)]
fn solve_part2(program: &[Instruction]) -> u32 {
    let mut computer = Computer::new();
    computer.set(Register::A, 1);

    computer.execute(&program);

    computer.get(Register::B)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_parse() {
        let cases = vec![
            ("hlf a", Instruction::Half(Register::A)),
            ("tpl b", Instruction::Triple(Register::B)),
            ("inc a", Instruction::Increment(Register::A)),
            ("jmp -32", Instruction::Jump(Offset(-32))),
            ("jie b, +2", Instruction::JumpIfEven(Register::B, Offset(2))),
            ("jio a, -2", Instruction::JumpIfOne(Register::A, Offset(-2))),
        ];

        for (input, expected) in cases {
            assert_eq!(Instruction::parse(input), expected);
        }
    }

    #[test]
    fn example_part1() {
        let program = vec![
            Instruction::Increment(Register::A),
            Instruction::JumpIfOne(Register::A, Offset(2)),
            Instruction::Triple(Register::A),
            Instruction::Increment(Register::A),
        ];

        let mut computer = Computer::new();

        computer.execute(&program);

        assert_eq!(computer.a, 2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day23.txt");

        let program = parse_input(input);

        assert_eq!(solve_part1(&program), 255);
        assert_eq!(solve_part2(&program), 334);
    }
}
