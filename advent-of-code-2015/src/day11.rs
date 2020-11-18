use aoc_runner_derive::aoc;
use fancy_regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Eq, PartialEq)]
enum NextChar {
    Next(char),
    WithOverflow(char),
}

fn next(c: char) -> NextChar {
    assert!(c.is_ascii_alphabetic() && c.is_lowercase());

    if c == 'z' {
        return NextChar::WithOverflow('a');
    }

    let next = ((c as u8) + 1) as char;
    NextChar::Next(next)
}

fn str_next(s: &str) -> String {
    assert!(s.is_ascii());

    let mut s = s.to_string();

    fn increment_str(s: &mut str) {
        let len = s.len();

        let bytes = s.as_bytes();
        let last_char = bytes[len - 1];

        let mut overflow_occurred = false;

        let last_char = match next(last_char as char) {
            NextChar::Next(c) => c,
            NextChar::WithOverflow(c) => {
                overflow_occurred = true;
                c
            }
        } as u8;

        // safety: manipulating the bytes directly is safe because all chars are ASCII
        unsafe {
            let bytes = s.as_bytes_mut();
            bytes[len - 1] = last_char as u8;
        }

        if overflow_occurred {
            increment_str(&mut s[0..len - 1]);
        }
    }

    increment_str(&mut s);

    s
}

#[derive(Debug, Eq, PartialEq)]
enum PasswordValidationError {
    InvalidCharacter(char),
    MissingThreeStraightIncreasingLetters,
    MissingTwoPairsOfLetters,
    PairsOverlap,
    PairsOfLettersAreNotDifferent,
}

type PasswordValidationResult = Result<(), PasswordValidationError>;

fn is_valid(s: &str) -> PasswordValidationResult {
    contains_invalid_characters(s)?;
    contains_three_straight_increasing_letters(s)?;
    contains_two_different_non_overlapping_pairs(s)?;
    does_not_contain_overlapping_pairs(s)?;

    Ok(())
}

fn contains_invalid_characters(s: &str) -> PasswordValidationResult {
    let invalid_chars = &['i', 'l', 'o'];

    for &c in invalid_chars {
        if s.contains(c) {
            return Err(PasswordValidationError::InvalidCharacter(c));
        }
    }

    Ok(())
}

fn contains_three_straight_increasing_letters(s: &str) -> PasswordValidationResult {
    let chars: Vec<char> = s.chars().collect();

    let any = chars
        .windows(3)
        .any(|c| {
            let c1 = c[0];
            let c2 = c[1];
            let c3 = c[2];

            matches!(next(c1), NextChar::Next(c) if c == c2)
                && matches!(next(c2), NextChar::Next(c) if c == c3)
        });

    if any {
        Ok(())
    } else {
        Err(PasswordValidationError::MissingThreeStraightIncreasingLetters)
    }
}

fn contains_two_different_non_overlapping_pairs(s: &str) -> PasswordValidationResult {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.)\1.*(.)\2").unwrap();
    }

    let captures = match RE.captures(s).unwrap() {
        Some(captures) => captures,
        None => return Err(PasswordValidationError::MissingTwoPairsOfLetters),
    };

    let capture1 = captures.get(1).unwrap();
    let capture2 = captures.get(2).unwrap();

    if capture1.as_str() == capture2.as_str() {
        return Err(PasswordValidationError::PairsOfLettersAreNotDifferent);
    }

    Ok(())
}

fn does_not_contain_overlapping_pairs(s: &str) -> PasswordValidationResult {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.)\1\1").unwrap();
    }

    if RE.is_match(s).unwrap() {
        return Err(PasswordValidationError::PairsOverlap);
    }

    Ok(())
}

fn find_next_valid(input: &str) -> String {
    let mut value = input.to_string();

    loop {
        value = str_next(&value);

        if is_valid(&value).is_ok() {
            return value;
        }
    }
}

#[aoc(day11, part1)]
fn solve_part1(input: &str) -> String {
    find_next_valid(input)
}

#[aoc(day11, part2)]
fn solve_part2(input: &str) -> String {
    find_next_valid(&find_next_valid(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_next_test() {
        assert_eq!(next('x'), NextChar::Next('y'));
        assert_eq!(next('y'), NextChar::Next('z'));
        assert_eq!(next('z'), NextChar::WithOverflow('a'));
        assert_eq!(next('a'), NextChar::Next('b'));
    }

    #[test]
    fn str_next_test() {
        assert_eq!(str_next("xx"), "xy");
        assert_eq!(str_next("xy"), "xz");
        assert_eq!(str_next("xz"), "ya");
        assert_eq!(str_next("ya"), "yb");
    }

    #[test]
    fn is_valid_test() {
        #[rustfmt::skip]
        let examples = vec![
            ("iaaxxyzz", Err(PasswordValidationError::InvalidCharacter('i'))),
            ("alaxxyzz", Err(PasswordValidationError::InvalidCharacter('l'))),
            ("aaoxxyzz", Err(PasswordValidationError::InvalidCharacter('o'))),
            ("abbceffg", Err(PasswordValidationError::MissingThreeStraightIncreasingLetters)),
            ("abbcexyz", Err(PasswordValidationError::MissingTwoPairsOfLetters)),
            ("abbbxyzz", Err(PasswordValidationError::PairsOverlap)),
            ("bbabbxyz", Err(PasswordValidationError::PairsOfLettersAreNotDifferent)),
            ("abcdffaa", Ok(())),
            ("ghjaabcc", Ok(())),
        ];

        for (input, expected) in examples {
            assert_eq!(is_valid(input), expected);
        }
    }

    #[test]
    fn examples_part1() {
        #[rustfmt::skip]
        let examples = vec![
            ("abcdefgh", "abcdffaa"),
            ("ghijklmn", "ghjaabcc")
        ];

        for (input, next_valid) in examples {
            assert_eq!(solve_part1(input), next_valid);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day11.txt").trim_end();

        assert_eq!(solve_part1(input), "hepxxyzz");
        assert_eq!(solve_part2(input), "heqaabcc");
    }
}
