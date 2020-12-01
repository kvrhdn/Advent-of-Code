use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u32> {
    let mut input: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    input.sort_unstable();
    input
}

#[aoc(day1, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    input
        .iter()
        .find(|&entry| input.binary_search(&(2020 - entry)).is_ok())
        .map(|entry| entry * (2020 - entry))
        .unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    input
        .iter()
        .tuple_combinations::<(&u32, &u32)>()
        .filter(|&(v1, v2)| v1 + v2 < 2020)
        .find(|&(v1, v2)| input.binary_search(&(2020 - v1 - v2)).is_ok())
        .map(|(v1, v2)| v1 * v2 * (2020 - v1 - v2))
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
