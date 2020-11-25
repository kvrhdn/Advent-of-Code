use common::console_utils::Timer;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/4
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part1");

    let range = parse_input(input)?;

    let valid_passwords = range
        .filter(|&p| has_six_digits(p))
        .filter(|&p| has_same_adjacent_digits(p))
        .filter(|&p| has_digits_that_never_decrease(p))
        .count() as u32;

    Ok(valid_passwords)
}

/// See: https://adventofcode.com/2019/day/4#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part2");

    let range = parse_input(input)?;

    let valid_passwords = range
        .filter(|&p| has_six_digits(p))
        .filter(|&p| has_at_least_one_double_same_adjacent_digits(p))
        .filter(|&p| has_digits_that_never_decrease(p))
        .count() as u32;

    Ok(valid_passwords)
}

fn parse_input(input: &str) -> Result<std::ops::Range<u32>, &'static str> {
    let range = input
        .trim_end()
        .split('-')
        .map(|l| {
            l.parse::<u32>()
                .map_err(|_| "could not parse input as integers")
        })
        .collect::<Result<Vec<u32>, &'static str>>()?;      // for some reason we have to specify the type here...

    if range.len() != 2 {
        return Err("expected input to contain exactly two integers");
    }
    let lower = range[0];
    let upper = range[1];

    Ok(lower..upper)
}

fn has_six_digits(num: u32) -> bool {
    num >= 100_000 && num <= 999_999
}

fn has_same_adjacent_digits(num: u32) -> bool {
    iterate_over_digits(num, &mut |left, right| left == right)
}

fn has_at_least_one_double_same_adjacent_digits(num: u32) -> bool {
    let mut digit_frequency = HashMap::<u32, u32>::new();

    iterate_over_digits(num, &mut |left, right| {
        // only add adjacent digits that are the same to the frequency map
        if left == right {
            digit_frequency.entry(left)
                .and_modify(|freq| *freq += 1)
                .or_insert(2);
        }

        false   // never stop iterating
    });

    // at least one digit should have a frequency of exactly 2
    digit_frequency.into_iter().any(|(_, freq)| freq == 2)
}

fn has_digits_that_never_decrease(num: u32) -> bool {
    iterate_over_digits(num, &mut |left, right| left > right)
        .inverted()
}

/// Iterates over the digits of integer from right to left, calling predicate
/// with both digits. The predicate is called with the left most digit as first
/// parameter.
/// Stops iterating if the predicate returns true or there are no digits left.
fn iterate_over_digits<F>(num: u32, predicate: &mut F) -> bool
where F: FnMut(u32, u32) -> bool
{
    let mut remainder = num;

    let mut prev_last_digit = remainder % 10;
    remainder /= 10;

    loop {
        let last_digit = remainder % 10;

        if predicate(last_digit, prev_last_digit) {
            return true;
        }

        prev_last_digit = last_digit;
        remainder /= 10;

        if remainder == 0 {
            return false;
        }
    }
}

trait BoolToggleExt {
    fn inverted(self) -> Self;
}

impl BoolToggleExt for bool {
    fn inverted(self) -> Self {
        !self
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;

    #[test]
    fn test_has_six_digits() {
        let cases = vec![
            (12345, false),
            (123456, true),
            (1234567, false),
        ];
        table_test(cases, has_six_digits, "has_six_digits");
    }

    #[test]
    fn test_has_same_adjacent_digits() {
        let cases = vec![
            (123456, false),
            (122345, true),
            (111111, true),
        ];
        table_test(cases, has_same_adjacent_digits, "has_same_adjacent_digits");
    }

    #[test]
    fn test_has_at_least_one_double_same_adjacent_digits() {
        let cases = vec![
            (123456, false),
            (112233, true),
            (123444, false),
            (111122, true),
        ];
        table_test(cases, has_at_least_one_double_same_adjacent_digits, "has_at_least_one_double_same_adjacent_digits");
    }

    #[test]
    fn test_has_digits_that_never_decrease() {
        let cases = vec![
            (123456, true),
            (111111, true),
            (123345, true),
            (123123, false),
        ];
        table_test(cases, has_digits_that_never_decrease, "has_digits_that_never_decrease");
    }

    fn table_test<F>(cases: Vec<(u32, bool)>, func: F, name: &str)
    where F: Fn(u32) -> bool
    {
        for (input, expected) in cases {
            let got = func(input);
            if got != expected {
                panic!("{}(\"{}\"): got {} but expected {}", name, input, got, expected);
            }            
        }
    }
}
