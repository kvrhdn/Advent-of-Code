use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashSet;

fn parse_program(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|line| {
            let (ir, value) = line.split(' ').collect_tuple().unwrap();

            (ir, value.parse().unwrap())
        })
        .collect()
}

// Execute the program and return the value of the accumulator and whether the
// program executed cleanly (i.e. not stuck in a loop).
fn execute(program: &[(&str, i32)]) -> (i32, bool) {
    let mut acc = 0;
    let mut ptr = 0;

    let mut visited = HashSet::new();

    loop {
        // loop detector
        if !visited.insert(ptr) {
            return (acc, false);
        }

        if ptr < 0 || ptr >= program.len() as i32 {
            return (acc, true);
        }

        let (ir, value) = program[ptr as usize];

        match ir {
            "nop" => {}
            "acc" => acc += value,
            "jmp" => ptr += value - 1,
            _ => panic!("unexpected instruction {}", ir),
        }

        ptr += 1;
    }
}

#[aoc(day8, part1)]
fn solve_part1(input: &str) -> i32 {
    let program = parse_program(input);

    execute(&program).0
}

#[aoc(day8, part2)]
fn solve_part2(input: &str) -> i32 {
    let program = parse_program(input);

    let manipulated_programs = (0..program.len())
        .filter_map(|index| {
            let (ir, _) = program[index];

            if ir != "nop" && ir != "jmp" {
                return None;
            }

            let mut manipulated_program = program.clone();

            let (ir, _) = manipulated_program.get_mut(index).unwrap();
            *ir = match *ir {
                "nop" => "jmp",
                "jmp" => "nop",
                _ => unreachable!(),
            };

            Some(manipulated_program)
        })
        .collect::<Vec<_>>();

    for manipulated_program in manipulated_programs {
        let (acc, clean_exit) = execute(&manipulated_program);

        if clean_exit {
            return acc;
        }
    }

    panic!("No manipulated program executed cleanly!");
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

        assert_eq!(solve_part1(input), 5);
        assert_eq!(solve_part2(input), 8);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day8.txt");

        assert_eq!(solve_part1(input), 1675);
        assert_eq!(solve_part2(input), 1532);
    }
}
