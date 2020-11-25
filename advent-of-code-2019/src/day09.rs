use crate::intcode::*;
use aoc_runner_derive::aoc;

/// See: https://adventofcode.com/2019/day/9
#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> Result<i64, &'static str> {
    let mut computer = Computer::new_from_input(input)?;

    computer.run_diagnostics_test(1).map_err(|err| err.into())
}

/// See: https://adventofcode.com/2019/day/9#part2
#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> Result<i64, &'static str> {
    let mut computer = Computer::new_from_input(input)?;

    computer.put_input(2);
    computer.run()?;

    computer
        .get_output()
        .ok_or_else(|| "BOOST program did not output any values".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day9.txt");

        assert_eq!(solve_part1(input), Ok(4261108180));
        assert_eq!(solve_part2(input), Ok(77944));
    }
}
