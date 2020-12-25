use aoc_runner_derive::aoc;
use itertools::Itertools;

fn find_loop_size(subject_number: u64, target: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;

    while value != target {
        value *= subject_number;
        value %= 20201227;

        loop_size += 1;
    }

    loop_size
}

fn transform_subject_number(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;

    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }

    value
}

#[aoc(day25, part1)]
fn solve_part1(input: &str) -> u64 {
    let (door_public_key, card_public_key) = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_tuple()
        .unwrap();

    let door_loop_size = find_loop_size(7, door_public_key);
    let card_loop_size = find_loop_size(7, card_public_key);

    assert_eq!(
        transform_subject_number(door_public_key, card_loop_size),
        transform_subject_number(card_public_key, door_loop_size),
    );

    transform_subject_number(card_public_key, door_loop_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
17807724
5764801";

        assert_eq!(solve_part1(input), 14897079);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day25.txt");

        assert_eq!(solve_part1(input), 354320);
    }
}
