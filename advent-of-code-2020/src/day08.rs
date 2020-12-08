use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;
use ExecutionResult::{InfiniteLoopDetected, Terminated};

#[derive(Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let (ir, value) = s.splitn(2, ' ').collect_tuple().unwrap();

        let value = value.parse().unwrap();

        match ir {
            "acc" => Instruction::Acc(value),
            "jmp" => Instruction::Jmp(value),
            "nop" => Instruction::Nop(value),
            _ => panic!("unsupported instruction: {}", ir),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum ExecutionResult {
    InfiniteLoopDetected,
    Terminated,
}

fn execute(program: &[Instruction]) -> (i32, ExecutionResult) {
    let mut acc = 0;
    let mut ptr = 0;

    let mut visited = HashSet::new();

    loop {
        // loop detector
        if !visited.insert(ptr) {
            return (acc, InfiniteLoopDetected);
        }

        if ptr < 0 || ptr >= program.len() as i32 {
            return (acc, Terminated);
        }

        match program[ptr as usize] {
            Instruction::Acc(value) => acc += value,
            Instruction::Jmp(value) => ptr += value - 1,
            Instruction::Nop(_) => {}
        }

        ptr += 1;
    }
}

#[aoc_generator(day8)]
fn parse_program(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::parse).collect()
}

#[aoc(day8, part1)]
fn solve_part1(program: &[Instruction]) -> i32 {
    execute(&program).0
}

#[aoc(day8, part2)]
fn solve_part2(program: &[Instruction]) -> i32 {
    (0..program.len())
        .filter_map(|index| {
            if !matches!(program[index], Instruction::Nop(_) | Instruction::Jmp(_)) {
                return None;
            }

            let mut manipulated_program = program.to_vec();

            let ir = manipulated_program.get_mut(index).unwrap();
            *ir = match *ir {
                Instruction::Jmp(value) => Instruction::Nop(value),
                Instruction::Nop(value) => Instruction::Jmp(value),
                Instruction::Acc(_) => unreachable!(),
            };

            Some(manipulated_program)
        })
        .find_map(|program| match execute(&program) {
            (acc, Terminated) => Some(acc),
            (_, InfiniteLoopDetected) => None,
        })
        .expect("No manipulated program terminated.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let program = parse_program(input);

        assert_eq!(solve_part1(&program), 5);
        assert_eq!(solve_part2(&program), 8);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day8.txt");
        let program = parse_program(input);

        assert_eq!(solve_part1(&program), 1675);
        assert_eq!(solve_part2(&program), 1532);
    }
}
