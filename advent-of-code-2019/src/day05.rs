use crate::intcode::*;
use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let mut computer = Computer::new_from_input(input).unwrap();

    computer.run_diagnostics_test(1).unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let mut computer = Computer::new_from_input(input).unwrap();

    computer.run_diagnostics_test(5).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day5.txt");

        assert_eq!(solve_part1(input), 13978427);
        assert_eq!(solve_part2(input), 11189491);
    }
}
