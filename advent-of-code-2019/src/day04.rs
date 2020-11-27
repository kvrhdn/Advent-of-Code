use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
fn solve_part1(input: &str) -> usize {
    parse_input(input).filter(valid_part1).count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> usize {
    parse_input(input).filter(valid_part2).count()
}

fn parse_input(input: &str) -> std::ops::Range<u32> {
    let range = input
        .trim_end()
        .split('-')
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    range[0]..range[1]
}

fn valid_part1(number: &u32) -> bool {
    let mut atleast_one_double = false;

    let mut prev = number % 10;

    for digit in iter_digits(number / 10) {
        if prev == digit {
            atleast_one_double = true;
        }

        if digit > prev {
            return false;
        }

        prev = digit;
    }

    atleast_one_double
}

fn valid_part2(number: &u32) -> bool {
    let mut atleast_one_exact_double = false;
    let mut consecutive_pairs = 0;

    let mut prev = number % 10;

    for digit in iter_digits(number / 10) {
        if digit == prev {
            consecutive_pairs += 1;
        } else {
            if consecutive_pairs == 1 {
                atleast_one_exact_double = true;
            }
            consecutive_pairs = 0;
        }

        if digit > prev {
            return false;
        }

        prev = digit;
    }

    if consecutive_pairs == 1 {
        atleast_one_exact_double = true;
    }

    atleast_one_exact_double
}

// Creates an iterator over the digits of the given number, from least
// significant to most significant.
fn iter_digits(mut number: u32) -> impl Iterator<Item = u32> {
    std::iter::from_fn(move || {
        if number > 0 {
            let digit = number % 10;
            number /= 10;
            Some(digit)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        let examples = vec![
            (111111, true), //
            (223450, false),
            (123789, false),
        ];
        for (input, expected) in examples {
            assert_eq!(valid_part1(&input), expected);
        }
    }

    #[test]
    fn examples_part2() {
        let examples = vec![
            (112233, true), //
            (123444, false),
            (111122, true),
        ];
        for (input, expected) in examples {
            assert_eq!(valid_part2(&input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day4.txt");

        assert_eq!(solve_part1(input), 1767);
        assert_eq!(solve_part2(input), 1192);
    }
}
