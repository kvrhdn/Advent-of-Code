use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use maplit::hashmap;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Aunt {
    number: u32,
    properties: HashMap<String, u32>,
}

impl Aunt {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Sue (?P<nr>\d+): (?P<properties>.+)$").unwrap();
        }
        let captures = RE.captures(s).unwrap();

        let number = captures.name("nr").unwrap().as_str().parse().unwrap();

        let properties = captures.name("properties").unwrap().as_str();
        let properties = properties
            .split(", ")
            .map(|p| {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"(?P<prop>\w+): (?P<value>\d+)").unwrap();
                }
                let captures = RE.captures(p).unwrap();

                let prop = captures.name("prop").unwrap().as_str().to_string();
                let value = captures.name("value").unwrap().as_str().parse().unwrap();

                (prop, value)
            })
            .collect();

        Aunt { number, properties }
    }

    fn matches_v1(&self, query: &HashMap<String, u32>) -> bool {
        for (prop, wanted) in query {
            if let Some(value) = self.properties.get(prop) {
                if value != wanted {
                    return false;
                }
            }
        }
        true
    }

    fn matches_v2(&self, query: &HashMap<String, u32>) -> bool {
        for (prop, wanted) in query {
            if let Some(value) = self.properties.get(prop) {
                let matches = match prop.as_str() {
                    "cats" | "trees" => value > wanted,
                    "pomeranians" | "goldfish" => value < wanted,
                    _ => value == wanted,
                };

                if !matches {
                    return false;
                }
            }
        }
        true
    }
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Vec<Aunt> {
    input.lines().map(|l| Aunt::parse(l)).collect()
}

fn mfcsam_reading() -> HashMap<String, u32> {
    hashmap! {
        "children".to_string() => 3,
        "cats".to_string() => 7,
        "samoyeds".to_string() => 2,
        "pomeranians".to_string() => 3,
        "akitas".to_string() => 0,
        "vizslas".to_string() => 0,
        "goldfish".to_string() => 5,
        "trees".to_string() => 3,
        "cars".to_string() => 2,
        "perfumes".to_string() => 1,
    }
}

#[aoc(day16, part1)]
fn solve_part1(aunts: &[Aunt]) -> u32 {
    let mfcsam_reading = mfcsam_reading();

    aunts
        .iter()
        .find(|aunt| aunt.matches_v1(&mfcsam_reading))
        .unwrap()
        .number
}

#[aoc(day16, part2)]
fn solve_part2(aunts: &[Aunt]) -> u32 {
    let mfcsam_reading = mfcsam_reading();

    aunts
        .iter()
        .find(|aunt| aunt.matches_v2(&mfcsam_reading))
        .unwrap()
        .number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ingredients() {
        let examples = vec![
            (
                "Sue 18: children: 9, goldfish: 2, akitas: 10",
                Aunt {
                    number: 18,
                    properties: hashmap! {
                        "children".to_string() => 9,
                        "goldfish".to_string() => 2,
                        "akitas".to_string() => 10,
                    },
                },
            ),
            (
                "Sue 47: pomeranians: 10, cars: 7, trees: 2",
                Aunt {
                    number: 47,
                    properties: hashmap! {
                        "pomeranians".to_string() => 10,
                        "cars".to_string() => 7,
                        "trees".to_string() => 2,
                    },
                },
            ),
        ];

        for (input, expected) in examples {
            assert_eq!(Aunt::parse(input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day16.txt");

        let aunts = parse_input(input);

        assert_eq!(solve_part1(&aunts), 40);
        assert_eq!(solve_part2(&aunts), 241);
    }
}
