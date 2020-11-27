use crate::intcode::*;
use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let mut computer = Computer::new_from_input(input).unwrap();

    computer.run_diagnostics_test(1).unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let mut computer = Computer::new_from_input(input).unwrap();

    computer.put_input(2);
    computer.run().unwrap();

    computer.get_output().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day9.txt");

        assert_eq!(solve_part1(input), 4261108180);
        assert_eq!(solve_part2(input), 77944);
    }
}
