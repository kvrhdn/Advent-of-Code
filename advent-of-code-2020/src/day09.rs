use aoc_runner_derive::{aoc, aoc_generator};

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
    let invalid_number = solve_part1(&input);

    let prefix_sum = input
        .iter()
        .scan(0u64, |sum, value| {
            *sum += *value;
            Some(*sum)
        })
        .collect::<Vec<_>>();

    for window_size in 2..input.len() {
        for i in window_size..input.len() {
            let sum = prefix_sum[i] - prefix_sum[i - window_size];

            if sum == invalid_number {
                let (min, max) = input[i - window_size..i]
                    .iter()
                    .fold((u64::MAX, u64::MIN), |(min, max), &value| {
                        (min.min(value), max.max(value))
                    });

                return min + max;
            }
        }
    }

    panic!("could not find a weakness");
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
