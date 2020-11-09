use aoc_runner_derive::aoc;
use std::str;

fn chars_in_code(s: &str) -> usize {
    s.len()
}

fn chars_in_memory(s: &str) -> usize {
    let mut count = 0;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(c) = chars.next() {
                // discard next 2 hexadecimal codes
                if c == 'x' {
                    chars.next();
                    chars.next();
                }
            }
        }
        count += 1;
    }

    // subtract quotes from both ends, these aren't stored in memory
    count - 2
}

fn chars_in_encoded(s: &str) -> usize {
    // +2 for additional wrapping quotes
    s.chars().flat_map(|c| c.escape_default()).count() + 2
}

#[aoc(day8, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| chars_in_code(l) - chars_in_memory(l))
        .sum()
}

#[aoc(day8, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| chars_in_encoded(l) - chars_in_code(l))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day08::*;

    #[test]
    fn examples_characters_code() {
        let examples = vec![
            (r#""""#, 2),
            (r#""abc""#, 5),
            (r#""aaa\"aaa""#, 10),
            (r#""\x27""#, 6),
        ];

        for (s, count) in examples {
            assert_eq!(chars_in_code(&s), count, "input {}", s);
        }
    }

    #[test]
    fn examples_characters_in_mem() {
        let examples = vec![
            (r#""""#, 0),
            (r#""abc""#, 3),
            (r#""aaa\"aaa""#, 7),
            (r#""\x27""#, 1),
        ];

        for (s, count) in examples {
            assert_eq!(chars_in_memory(&s), count, "input: {}", s);
        }
    }

    #[test]
    fn examples_characters_in_encoded() {
        let examples = vec![
            (r#""""#, 6),
            (r#""abc""#, 9),
            (r#""aaa\"aaa""#, 16),
            (r#""\x27""#, 11),
        ];

        for (s, count) in examples {
            assert_eq!(chars_in_encoded(&s), count, "input: {}", s);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day8.txt");

        assert_eq!(solve_part1(input), 1333);
        assert_eq!(solve_part2(input), 2046);
    }
}
