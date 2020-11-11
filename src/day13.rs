use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct PotentialHappiness {
    subject: String,
    next_to: String,
    potential: i32,
}

impl PotentialHappiness {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(
                    r"^(?P<subject>\w+) would (?P<op>gain|lose) (?P<units>\d+) happiness units by sitting next to (?P<next_to>\w+).$"
                )
                    .unwrap();
        }
        let captures = RE.captures(s).unwrap();

        let value: i32 = captures.name("units").unwrap().as_str().parse().unwrap();
        let value = match captures.name("op").unwrap().as_str() {
            "gain" => value,
            "lose" => -value,
            invalid => panic!("Unexpected op {}", invalid),
        };

        PotentialHappiness {
            subject: captures.name("subject").unwrap().as_str().to_string(),
            next_to: captures.name("next_to").unwrap().as_str().to_string(),
            potential: value,
        }
    }
}

fn extract_guests(potentials: &[PotentialHappiness]) -> Vec<String> {
    let mut set = HashSet::new();

    for c in potentials {
        set.insert(&c.subject);
        set.insert(&c.next_to);
    }

    set.iter().map(|&v| v.clone()).collect()
}

fn find_total_happiness_between(
    potentials: &[PotentialHappiness],
    guest1: &str,
    guest2: &str,
) -> i32 {
    potentials
        .iter()
        .filter(|&c| {
            // count happiness in both directions
            (c.subject == guest1 && c.next_to == guest2)
                || (c.subject == guest2 && c.next_to == guest1)
        })
        .map(|c| c.potential)
        .sum()
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<PotentialHappiness> {
    input
        .lines()
        .map(|l| PotentialHappiness::parse(l))
        .collect::<Vec<_>>()
}

#[aoc(day13, part1)]
fn solve_part1(input: &[PotentialHappiness]) -> i32 {
    let guests = extract_guests(input);

    guests
        .iter()
        .permutations(guests.len())
        .map(|c| {
            let happiness: i32 = c
                .windows(2)
                .map(|w| find_total_happiness_between(input, w[0], w[1]))
                .sum();

            happiness + find_total_happiness_between(input, c.last().unwrap(), c.first().unwrap())
        })
        .max()
        .unwrap()
}

#[aoc(day13, part2)]
fn solve_part2(input: &[PotentialHappiness]) -> i32 {
    let mut input_with_self = input.to_vec();

    // it's sufficient to insert this dummy PotentialHappiness to add "me" to
    // the guest list and find_total_happiness_between will default to 0 if the
    // actual PotentialHappiness is not found
    input_with_self.push(PotentialHappiness {
        subject: "me".into(),
        next_to: "me".into(),
        potential: 0,
    });

    solve_part1(&input_with_self)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potential_happiness_parse() {
        let examples = vec![
            (
                "Alice would gain 54 happiness units by sitting next to Bob.",
                PotentialHappiness {
                    subject: "Alice".into(),
                    next_to: "Bob".into(),
                    potential: 54,
                },
            ),
            (
                "David would lose 7 happiness units by sitting next to Carol.",
                PotentialHappiness {
                    subject: "David".into(),
                    next_to: "Carol".into(),
                    potential: -7,
                },
            ),
        ];

        for (input, potential) in examples {
            assert_eq!(PotentialHappiness::parse(input), potential);
        }
    }

    #[test]
    fn example_part1() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

        let potentials = parse_input(input);

        assert_eq!(solve_part1(&potentials), 330);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day13.txt");

        let potentials = parse_input(input);

        assert_eq!(solve_part1(&potentials), 618);
        assert_eq!(solve_part2(&potentials), 601);
    }
}
