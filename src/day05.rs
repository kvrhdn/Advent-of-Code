use aoc_runner_derive::aoc;
use fancy_regex::Regex;
use lazy_static::lazy_static;

fn is_nice_v1(s: &str) -> bool {
    contains_at_least_3_vowels(s)
        && contains_at_least_one_double_letter(s)
        && does_not_contain_a_bad_string(s)
}

fn is_nice_v2(s: &str) -> bool {
    contains_pair_without_overlapping(s) && contains_a_sandwiching_pair(s)
}

fn contains_at_least_3_vowels(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.*[aeiou]){3}").unwrap();
    }
    RE.is_match(s).unwrap()
}

fn contains_at_least_one_double_letter(s: &str) -> bool {
    // due to backtracking, a regex is not faster than this
    let original_length = s.len();

    let mut vec = s.chars().collect::<Vec<_>>();
    vec.dedup();

    vec.len() != original_length
}

fn does_not_contain_a_bad_string(s: &str) -> bool {
    !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"))
}

fn contains_pair_without_overlapping(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((.){2}).*?(\1)").unwrap();
    }
    RE.is_match(s).unwrap()
}

fn contains_a_sandwiching_pair(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.)(.)\1").unwrap();
    }
    RE.is_match(s).unwrap()
}

#[aoc(day5, part1)]
fn solve_part1(input: &str) -> u32 {
    input.lines().filter(|&s| is_nice_v1(s)).count() as u32
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> u32 {
    input.lines().filter(|&s| is_nice_v2(s)).count() as u32
}

#[cfg(test)]
mod tests {
    use crate::day05::*;

    #[test]
    fn examples_is_nice_v1() {
        let examples = vec![
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ];

        for e in examples {
            assert_eq!(is_nice_v1(e.0), e.1);
        }
    }

    #[test]
    fn examples_is_nice_v2() {
        let examples = vec![
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
        ];

        for e in examples {
            assert_eq!(is_nice_v2(e.0), e.1);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day5.txt");

        assert_eq!(solve_part1(input), 236);
        assert_eq!(solve_part2(input), 51);
    }
}
