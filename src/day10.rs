use aoc_runner_derive::{aoc, aoc_generator};
use std::borrow::Borrow;

fn process_times<T, F>(times: u32, initial_value: &T, process: F) -> T
where
    T: Borrow<T>,
    F: Fn(&T) -> T,
{
    let mut value = process(initial_value);

    for _ in 2..=times {
        value = process(value.borrow());
    }

    value
}

fn look_and_say(value: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();

    let mut iter = value.iter().peekable();

    while let Some(c) = iter.next() {
        let mut count = 1;

        while let Some(next) = iter.peek() {
            if *next == c {
                iter.next();
                count += 1;
            } else {
                break;
            }
        }

        result.push(count);
        result.push(*c);
    }

    result
}

#[aoc_generator(day10)]
pub fn parse_input_as_numbers(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<u8>) -> usize {
    let result = process_times(40, input, |v| look_and_say(v));
    result.len()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<u8>) -> usize {
    let result = process_times(50, input, |v| look_and_say(v));
    result.len()
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    #[test]
    fn process_times_test() {
        assert_eq!(process_times(5, &0, |v| v + 1), 5);
    }

    #[test]
    fn examples_look_and_say() {
        let examples = vec![
            (vec![1], vec![1, 1]),
            (vec![1, 1], vec![2, 1]),
            (vec![2, 1], vec![1, 2, 1, 1]),
            (vec![1, 2, 1, 1], vec![1, 1, 1, 2, 2, 1]),
            (vec![1, 1, 1, 2, 2, 1], vec![3, 1, 2, 2, 1, 1]),
        ];

        for (input, result) in examples {
            assert_eq!(look_and_say(&input), result)
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day10.txt").trim_end();

        let input_numbers = parse_input_as_numbers(input);

        assert_eq!(solve_part1(&input_numbers), 360154);
        assert_eq!(solve_part2(&input_numbers), 5103798);
    }
}
