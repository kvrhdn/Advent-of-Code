use aoc_runner_derive::aoc;
use itertools::{EitherOrBoth::Both, EitherOrBoth::Left, EitherOrBoth::Right, Itertools};
use std::collections::HashMap;

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> u32 {
    let orbit_map = OrbitMap::build(input);

    orbit_map
        .all_orbitters()
        .map(|object| orbit_map.orbit_level(object) + 1)
        .sum()
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> u32 {
    let orbit_map = OrbitMap::build(input);

    let mut path_you = orbit_map.path_to_center("YOU");
    path_you.reverse();
    let mut path_san = orbit_map.path_to_center("SAN");
    path_san.reverse();

    path_you
        .iter()
        .zip_longest(path_san.iter())
        // filter out the common path of both paths
        .filter(|pair| match pair {
            Both(l, r) => l != r,
            _ => true,
        })
        // count how many objects remain
        .map(|pair| match pair {
            Both(_, _) => 2,
            Left(_) | Right(_) => 1,
        })
        .sum::<u32>()
        // subtract "YOU" and "SAN"
        - 2
}

struct OrbitMap {
    // hashmap with in_orbit -> at_center
    map: HashMap<String, String>,
}

impl OrbitMap {
    fn build(input: &str) -> Self {
        let map = input
            .trim_end()
            .lines()
            .map(|l| {
                let objects = l.split(')').collect::<Vec<&str>>();

                if objects.len() != 2 {
                    panic!("expected input to be of format x)y");
                }

                let at_center = (*objects.get(0).unwrap()).to_string();
                let in_orbit = (*objects.get(1).unwrap()).to_string();

                (in_orbit, at_center)
            })
            .collect();

        OrbitMap { map }
    }

    /// Returns an iterator over all objects in orbit.
    fn all_orbitters(&self) -> impl Iterator<Item = &String> {
        self.map.values()
    }

    /// Optionally returns the space object the given object orbits around.
    fn orbits_around(&self, object: &str) -> Option<&String> {
        self.map.get(object)
    }

    /// The orbit level is the amount of other objects it orbits around. The
    /// object with orbit level 0 is at the absolute center of the universe.
    fn orbit_level(&self, object: &str) -> u32 {
        match self.orbits_around(object) {
            Some(at_center) => 1 + self.orbit_level(at_center),
            None => 0,
        }
    }

    fn path_to_center<'a>(&'a self, object: &'a str) -> Vec<&'a str> {
        let mut path = Vec::new();
        let mut curr = object;
        loop {
            path.push(curr);

            curr = match self.orbits_around(curr) {
                Some(obj) => obj,
                None => break,
            }
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn build_orbit_map() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n";

        let expected_map = hashmap! {
            "B".to_string() => "COM".to_string(),
            "C".to_string() => "B".to_string(),
            "D".to_string() => "C".to_string(),
            "E".to_string() => "D".to_string(),
            "F".to_string() => "E".to_string(),
            "G".to_string() => "B".to_string(),
            "H".to_string() => "G".to_string(),
            "I".to_string() => "D".to_string(),
            "J".to_string() => "E".to_string(),
            "K".to_string() => "J".to_string(),
            "L".to_string() => "K".to_string(),
        };

        let orbit_map = OrbitMap::build(input);
        assert_eq!(orbit_map.map, expected_map);

        assert_eq!(orbit_map.orbit_level("D"), 3);
        assert_eq!(orbit_map.orbit_level("L"), 7);
        assert_eq!(orbit_map.orbit_level("COM"), 0);

    }

    #[test]
    fn example_part1() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n";

        assert_eq!(solve_part1(input), 42);
    }

    #[test]
    fn example_part2() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";

        assert_eq!(solve_part2(input), 4);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day6.txt");

        assert_eq!(solve_part1(input), 253104);
        assert_eq!(solve_part2(input), 499);
    }
}
