use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reindeer {
    pub name: String,
    pub speed: u32,         // in km/s
    pub fly_duration: u32,  // in s
    pub rest_duration: u32, // in s
}

impl Reindeer {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(
                    r"^(?P<name>\w+) can fly (?P<speed>\d+) km/s for (?P<fly>\d+) seconds, but then must rest for (?P<rest>\d+) seconds.$"
                )
                    .unwrap();
        }
        let captures = RE.captures(s).unwrap();

        Reindeer {
            name: captures.name("name").unwrap().as_str().to_string(),
            speed: captures.name("speed").unwrap().as_str().parse().unwrap(),
            fly_duration: captures.name("fly").unwrap().as_str().parse().unwrap(),
            rest_duration: captures.name("rest").unwrap().as_str().parse().unwrap(),
        }
    }

    fn distance_travelled_after(&self, seconds: u32) -> u32 {
        let mut remaining = seconds;
        let mut distance_travelled = 0;

        loop {
            // fly
            if self.fly_duration > remaining {
                distance_travelled += self.speed * remaining;
                break;
            }
            distance_travelled += self.speed * self.fly_duration;
            remaining -= self.fly_duration;

            // rest
            if self.rest_duration > remaining {
                break;
            }
            remaining -= self.rest_duration;
        }

        distance_travelled
    }
}

/// For every second, the reindeer that is in the lead at that moment receives
/// a point.
fn race_with_points_system(reindeer: &[Reindeer], duration: u32) -> HashMap<String, u32> {
    let mut points = HashMap::new();

    for r in reindeer {
        points.insert(r.name.clone(), 0);
    }

    for s in 1..=duration {
        // while recalculating the distance for every second is inefficient
        // (compared to using some mutable Reindeer struct), it's still plenty
        // fast 🤷‍♂️
        let state = reindeer
            .iter()
            .map(|r| (&r.name, r.distance_travelled_after(s)))
            .collect::<Vec<_>>();

        let lead = state.iter().map(|(_, distance)| distance).max().unwrap();

        for (name, distance) in &state {
            if distance == lead {
                let reindeer_points = points.get_mut(*name).unwrap();
                *reindeer_points += 1;
            }
        }
    }

    points
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Vec<Reindeer> {
    input.lines().map(|l| Reindeer::parse(l)).collect()
}

#[aoc(day14, part1)]
fn solve_part1(reindeer: &[Reindeer]) -> u32 {
    reindeer
        .iter()
        .map(|r| r.distance_travelled_after(2503))
        .max()
        .unwrap()
}

#[aoc(day14, part2)]
fn solve_part2(reindeer: &[Reindeer]) -> u32 {
    let points = race_with_points_system(reindeer, 2503);

    *points.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potential_happiness_parse() {
        let examples = vec![
            (
                "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
                Reindeer {
                    name: "Comet".into(),
                    speed: 14,
                    fly_duration: 10,
                    rest_duration: 127,
                },
            ),
            (
                "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
                Reindeer {
                    name: "Dancer".into(),
                    speed: 16,
                    fly_duration: 11,
                    rest_duration: 162,
                },
            ),
        ];

        for (input, reindeer) in examples {
            assert_eq!(Reindeer::parse(input), reindeer);
        }
    }

    #[test]
    fn example_part1() {
        let comet = Reindeer::parse(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        );
        let dancer = Reindeer::parse(
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        );

        assert_eq!(comet.distance_travelled_after(1000), 1120);
        assert_eq!(dancer.distance_travelled_after(1000), 1056);
    }

    #[test]
    fn example_part2() {
        let comet = Reindeer::parse(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        );
        let dancer = Reindeer::parse(
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        );

        let points = race_with_points_system(&[comet, dancer], 1000);

        assert_eq!(points.get("Comet"), Some(&312));
        assert_eq!(points.get("Dancer"), Some(&689));
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day14.txt");

        let reindeer = parse_input(input);

        assert_eq!(solve_part1(&reindeer), 2660);
        assert_eq!(solve_part2(&reindeer), 1256);
    }
}
