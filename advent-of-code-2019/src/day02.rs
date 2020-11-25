use aoc_runner_derive::aoc;
use crate::intcode::*;

/// See: https://adventofcode.com/2019/day/2
#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> Result<i64, &'static str> {
    let mut computer = Computer::new_from_input(input)?;
    set_noun_and_verb(&mut computer, 12, 2);

    computer.run()?;

    Ok(computer.get(0))
}

/// See: https://adventofcode.com/2019/day/2#part2
#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> Result<i64, &'static str> {
    let program = load_program(input)?;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = Computer::new(program.clone());
            set_noun_and_verb(&mut computer, noun, verb);

            computer.run()?;

            if computer.get(0) == 19_690_720 {
                return Ok((100 * noun) + verb);
            }
        }
    }

    Err("could not find a noun and verb".into())
}

fn set_noun_and_verb(computer: &mut Computer, noun: i64, verb: i64) {
    computer.set(1, noun);
    computer.set(2, verb);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day2.txt");

        assert_eq!(solve_part1(input), Ok(4484226));
        assert_eq!(solve_part2(input), Ok(5696));
    }
}
