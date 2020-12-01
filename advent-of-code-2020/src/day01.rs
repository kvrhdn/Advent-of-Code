use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1, vec_combinations)]
fn solve_part1_combinations(input: &[u32]) -> u32 {
    input
        .iter()
        .copied()
        .combinations(2)
        .filter(|values| values.iter().sum::<u32>() == 2020)
        .map(|values| values.iter().product())
        .next()
        .unwrap()
}

#[aoc(day1, part1, tuple_combinations)]
fn solve_part1_tuple_combinations(input: &[u32]) -> u32 {
    input
        .iter()
        .tuple_combinations::<(&u32, &u32)>()
        .filter(|&(v1, v2)| v1 + v2 == 2020)
        .map(|(v1, v2)| v1 * v2)
        .next()
        .unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    input
        .iter()
        .tuple_combinations::<(&u32, &u32, &u32)>()
        .filter(|(&v1, &v2, &v3)| v1 + v2 + v3 == 2020)
        .map(|(v1, v2, v3)| v1 * v2 * v3)
        .next()
        .unwrap()
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

        assert_eq!(solve_part1_combinations(&input), 514579);
        assert_eq!(solve_part1_tuple_combinations(&input), 514579);
        assert_eq!(solve_part2(&input), 241861950);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day1.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1_combinations(&input), 381699);
        assert_eq!(solve_part1_tuple_combinations(&input), 381699);
        assert_eq!(solve_part2(&input), 111605670);
    }
}
