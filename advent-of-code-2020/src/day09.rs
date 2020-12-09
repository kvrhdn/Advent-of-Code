use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, MinMaxResult::MinMax};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_sum(preamble: &[u64], sum: u64) -> bool {
    preamble
        .iter()
        .any(|&value| value < sum && preamble.contains(&(sum - value)))
}

#[aoc(day9, part1)]
fn solve_part1(input: &[u64]) -> u64 {
    input
        .windows(26)
        .filter_map(|window| {
            let (sum, preamble) = window.split_last().unwrap();

            if find_sum(preamble, *sum) {
                None
            } else {
                Some(*sum)
            }
        })
        .next()
        .unwrap()
}

#[aoc(day9, part2)]
fn solve_part2(input: &[u64]) -> u64 {
    let invalid_number = solve_part1(input);

    let mut sum = 0;
    let mut start = 0;
    let mut end = 0;

    loop {
        while sum < invalid_number {
            sum += input[end];
            end += 1;
        }

        while sum > invalid_number {
            sum -= input[start];
            start += 1;
        }

        if sum == invalid_number {
            break;
        }
    }

    match input[start..end].iter().minmax() {
        MinMax(min, max) => min + max,
        _ => panic!("expected a min and a max value"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day9.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 542529149);
        assert_eq!(solve_part2(&input), 75678618);
    }
}
