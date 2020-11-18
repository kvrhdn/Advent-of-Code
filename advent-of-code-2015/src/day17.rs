use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn volume_of(containers: &[&u32]) -> u32 {
    containers.iter().copied().sum()
}

fn all_combinations_to_store(containers: &[u32], desired_size: u32) -> Vec<Vec<&u32>> {
    (1..=containers.len())
        .flat_map(|length| containers.iter().combinations(length))
        .filter(|combination| volume_of(combination) == desired_size)
        .collect()
}

fn smallest_combinations_to_store(containers: &[u32], desired_size: u32) -> Vec<Vec<&u32>> {
    let all_combinations = all_combinations_to_store(containers, desired_size);

    let smallest = all_combinations
        .iter()
        .map(|combination| combination.len())
        .min()
        .unwrap();

    all_combinations
        .into_iter()
        .filter(|combination| combination.len() == smallest)
        .collect()
}

#[aoc(day17, part1)]
fn solve_part1(containers: &[u32]) -> usize {
    all_combinations_to_store(containers, 150).len()
}

#[aoc(day17, part2)]
fn solve_part2(containers: &[u32]) -> usize {
    smallest_combinations_to_store(containers, 150).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "20\n15\n10\n5\n5";

        let containers = parse_input(input);

        assert_eq!(all_combinations_to_store(&containers, 25).len(), 4);
        assert_eq!(smallest_combinations_to_store(&containers, 25).len(), 3);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day17.txt");

        let containers = parse_input(input);

        assert_eq!(solve_part1(&containers), 1638);
        assert_eq!(solve_part2(&containers), 17);
    }
}
