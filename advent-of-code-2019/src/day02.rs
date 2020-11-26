use crate::intcode::{load_program, Computer};
use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn solve_part1(input: &str) -> i64 {
    let mut computer = Computer::new_from_input(input).unwrap();
    set_noun_and_verb(&mut computer, 12, 2);

    computer.run().unwrap();

    computer.get(0)
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> i64 {
    let program = load_program(input).unwrap();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = Computer::new(program.clone());
            set_noun_and_verb(&mut computer, noun, verb);

            computer.run().unwrap();

            if computer.get(0) == 19_690_720 {
                return (100 * noun) + verb;
            }
        }
    }

    panic!("could not find a noun and verb")
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

        assert_eq!(solve_part1(input), 4484226);
        assert_eq!(solve_part2(input), 5696);
    }
}
