use aoc_runner_derive::aoc;
use std::collections::HashMap;

const U32_MAX: u64 = 0xFFFFFFFFF;

#[derive(Debug)]
enum Instruction {
    Mask(Bitmask),
    Mem(usize, u64),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        if let Ok((addr, value)) = serde_scan::scan!("mem[{}] = {}" <- s) {
            Instruction::Mem(addr, value)
        } else {
            Instruction::Mask(Bitmask::parse(&s[7..]))
        }
    }
}

#[derive(Debug)]
struct Bitmask {
    ones: u64,   // example: `value & 0010` will always set the second bit
    zeroes: u64, // example: `value ^ 1011` will always zero the third bit
}

impl Bitmask {
    fn parse(s: &str) -> Self {
        let (ones, zeroes) =
            s.as_bytes()
                .iter()
                .enumerate()
                .fold((0u64, U32_MAX), |(ones, zeroes), (i, v)| {
                    let index = 36 - i - 1;
                    match v {
                        b'0' => (ones, zeroes ^ 1 << index),
                        b'1' => (ones | 1 << index, zeroes),
                        b'X' => (ones, zeroes),
                        _ => panic!("Unsupported binary value {:b}", v),
                    }
                });

        Self { ones, zeroes }
    }

    fn apply(&self, value: u64) -> u64 {
        (value | self.ones) & self.zeroes
    }
}

impl Default for Bitmask {
    fn default() -> Self {
        Bitmask {
            ones: 0,
            zeroes: U32_MAX,
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| Instruction::parse(line))
}

#[aoc(day14, part1)]
fn solve_part1(input: &str) -> u64 {
    let instructions = parse_input(input);

    let mut mask = Default::default();
    let mut mem = HashMap::new();

    for ir in instructions {
        match ir {
            Instruction::Mask(new_mask) => {
                mask = new_mask;
            }
            Instruction::Mem(addr, value) => {
                mem.insert(addr, mask.apply(value));
            }
        }
    }

    mem.values().sum()
}

#[aoc(day14, part2)]
fn solve_part2(input: &str) -> i32 {
    // TODO
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        assert_eq!(solve_part1(input), 165);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day14.txt");

        assert_eq!(solve_part1(input), 5875750429995);
        assert_eq!(solve_part2(input), 0);
    }
}
