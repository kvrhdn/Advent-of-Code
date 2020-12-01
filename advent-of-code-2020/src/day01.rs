use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    product_of_entries_that_make_2020(input, 2)
}

#[aoc(day1, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    product_of_entries_that_make_2020(input, 3)
}

fn product_of_entries_that_make_2020(entries: &[u32], window_size: usize) -> u32 {
    entries
        .iter()
        .combinations(window_size)
        .find(|values| values.iter().copied().sum::<u32>() == 2020)
        .unwrap()
        .into_iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"1721
979
366
299
675
1456"#;
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 514579);
        assert_eq!(solve_part2(&input), 241861950);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day1.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 381699);
        assert_eq!(solve_part2(&input), 111605670);
    }
}
