use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut answers = group
                .chars()
                .filter(|c| !c.is_ascii_whitespace())
                .collect::<Vec<_>>();
            answers.sort_unstable();
            answers.dedup();
            answers
        })
        .map(|group| group.len())
        .sum()
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut all_answers = group.lines();
            let mut common = all_answers.next().unwrap().chars().collect::<Vec<_>>();

            for answers in all_answers {
                // remove all answers that are not shared
                common.retain(|&c| answers.contains(c));
            }

            common.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(solve_part1(input), 11);
        assert_eq!(solve_part2(input), 6);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day6.txt");

        assert_eq!(solve_part1(input), 6291);
        assert_eq!(solve_part2(input), 3052);
    }
}
