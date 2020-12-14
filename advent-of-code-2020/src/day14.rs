use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;
use std::collections::HashMap;

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

struct Bitmask {
    zeroes_mask: u64,
    ones_mask: u64,
    floating: ArrayVec<[usize; 40]>,
}

impl Bitmask {
    fn parse(s: &str) -> Self {
        let mut bitmask: Bitmask = Default::default();

        for (i, b) in s.as_bytes().iter().enumerate() {
            let index = 36 - i - 1;

            match b {
                b'0' => bitmask.zeroes_mask |= 1 << index,
                b'1' => bitmask.ones_mask |= 1 << index,
                b'X' => bitmask.floating.push(index),
                _ => panic!("Unsupported value {:b}", b),
            };
        }

        bitmask
    }

    fn apply_mask_to_value(&self, value: u64) -> u64 {
        (value | self.ones_mask) & !self.zeroes_mask
    }

    fn decode_address(&self, addr: u64) -> impl Iterator<Item = u64> + '_ {
        let addr = addr | self.ones_mask;

        self.floating
            .iter()
            .fold(vec![addr], |addresses, floating| {
                let mut new = Vec::new();

                for addr in addresses {
                    new.push(addr | 1 << floating);
                    new.push(addr & !(1 << floating));
                }

                new
            })
            .into_iter()
    }
}

impl Default for Bitmask {
    fn default() -> Self {
        Bitmask {
            ones_mask: 0,
            zeroes_mask: 0,
            floating: Default::default(),
        }
    }
}

fn process_instructions<F>(input: &str, exec_mem: F) -> u64
where
    F: Fn(&mut HashMap<usize, u64>, &Bitmask, usize, u64),
{
    let instructions = input.lines().map(|line| Instruction::parse(line));

    let mut mask = Default::default();
    let mut mem = HashMap::new();

    for ir in instructions {
        match ir {
            Instruction::Mask(new_mask) => {
                mask = new_mask;
            }
            Instruction::Mem(addr, value) => {
                exec_mem(&mut mem, &mask, addr, value);
            }
        }
    }

    mem.values().sum()
}

#[aoc(day14, part1)]
fn solve_part1(input: &str) -> u64 {
    process_instructions(input, |mem, mask, addr, value| {
        mem.insert(addr, mask.apply_mask_to_value(value));
    })
}

#[aoc(day14, part2)]
fn solve_part2(input: &str) -> u64 {
    process_instructions(input, |mem, mask, addr, value| {
        for addr in mask.decode_address(addr as u64) {
            mem.insert(addr as usize, value);
        }
    })
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
    fn example_part2() {
        let input = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        assert_eq!(solve_part2(input), 208);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day14.txt");

        assert_eq!(solve_part1(input), 5875750429995);
        assert_eq!(solve_part2(input), 5272149590143);
    }
}
