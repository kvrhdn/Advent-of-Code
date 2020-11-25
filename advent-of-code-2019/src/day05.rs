use aoc_runner_derive::aoc;
use crate::intcode::*;

/// See: https://adventofcode.com/2019/day/5
#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> Result<i64, &'static str> {
    let mut computer = Computer::new_from_input(input)?;

    computer.run_diagnostics_test(1)
        .map_err(|err| err.into())
}

/// See: https://adventofcode.com/2019/day/5#part2
#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> Result<i64, &'static str> {
    let mut computer = Computer::new_from_input(input)?;

    computer.run_diagnostics_test(5)
        .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day5.txt");

        assert_eq!(solve_part1(input), Ok(13978427));
        assert_eq!(solve_part2(input), Ok(11189491));
    }
}
