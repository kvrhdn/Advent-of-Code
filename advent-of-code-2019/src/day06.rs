use aoc_runner_derive::aoc;
use std::collections::HashMap;

/// See: https://adventofcode.com/2019/day/6
#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> Result<u32, &'static str> {
    let orbit_map = OrbitMap::build(input)?;

    let orbit_count: u32 = orbit_map
        .get_all_orbitters()
        .iter()
        .map(|object| orbit_map.orbit_level(object.to_string()))
        .sum();

    let orbitters_count = orbit_map.get_all_orbitters().len() as u32;

    // TODO it's not entirely clear to me why we need to add the amount of orbitters as well... D:
    Ok(orbit_count + orbitters_count)
}

/// See: https://adventofcode.com/2019/day/6#part2
#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> Result<u32, &'static str> {
    let orbit_map = OrbitMap::build(input)?;

    let path = orbit_map.path_to_center("YOU".into());

    let mut curr: SpaceObject = "SAN".into();
    let mut steps = 0;
    loop {
        curr = match orbit_map.orbits_around(curr) {
            Some(obj) => obj,
            None => return Err("reached center before finding an intersection".into()),
        };
        steps += 1;

        if let Some((index, _)) = path.iter().enumerate().find(|(_, obj)| **obj == curr) {
            // TODO it's not entirely clear to me why we need to subtract 2 as well... D:
            return Ok(steps + index as u32 - 2);
        }
    }
}

type SpaceObject = String;

struct OrbitMap {
    map: HashMap<String, String>,
}

impl OrbitMap {
    fn build(input: &str) -> Result<Self, &'static str> {
        let map = input
            .trim_end()
            .lines()
            .map(|l| {
                let objects = l.split(')').collect::<Vec<&str>>();

                if objects.len() != 2 {
                    return Err("expected input to be of format x)y");
                }

                let at_center = (*objects.get(0).unwrap()).to_string();
                let in_orbit = (*objects.get(1).unwrap()).to_string();

                Ok((in_orbit, at_center))
            })
            .collect::<Result<HashMap<SpaceObject, SpaceObject>, _>>()?;

        Ok(OrbitMap { map })
    }

    /// Returns a list of all objects in orbit around some object.
    fn get_all_orbitters(&self) -> Vec<SpaceObject> {
        self.map.values().cloned().collect()
    }

    /// Optionally returns the space object the given object orbits around.
    /// Completes in O(1) time.
    fn orbits_around(&self, object: SpaceObject) -> Option<SpaceObject> {
        self.map.get(&object).cloned()
    }

    /// The orbit level is the amount of other objects it orbits around. The
    /// object with orbit level 0 is at the absolute center of the universe.
    fn orbit_level(&self, object: SpaceObject) -> u32 {
        match self.orbits_around(object) {
            Some(at_center) => 1 + self.orbit_level(at_center),
            None => 0,
        }
    }

    fn path_to_center(&self, object: SpaceObject) -> Vec<SpaceObject> {
        let mut path = Vec::new();
        let mut curr = object;
        loop {
            path.push(curr.clone());

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

    #[test]
    fn example_part1() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n";

        let mut map = HashMap::new();
        map.entry("B".into()).or_insert_with(|| "COM".into());
        map.entry("C".into()).or_insert_with(|| "B".into());
        map.entry("D".into()).or_insert_with(|| "C".into());
        map.entry("E".into()).or_insert_with(|| "D".into());
        map.entry("F".into()).or_insert_with(|| "E".into());
        map.entry("G".into()).or_insert_with(|| "B".into());
        map.entry("H".into()).or_insert_with(|| "G".into());
        map.entry("I".into()).or_insert_with(|| "D".into());
        map.entry("J".into()).or_insert_with(|| "E".into());
        map.entry("K".into()).or_insert_with(|| "J".into());
        map.entry("L".into()).or_insert_with(|| "K".into());

        let orbit_map = match OrbitMap::build(input) {
            Ok(orbit_map) => orbit_map,
            Err(err) => panic!(err),
        };
        assert_eq!(orbit_map.map, map);

        assert_eq!(orbit_map.orbit_level("D".to_string()), 3);
        assert_eq!(orbit_map.orbit_level("L".to_string()), 7);
        assert_eq!(orbit_map.orbit_level("COM".to_string()), 0);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day6.txt");

        assert_eq!(solve_part1(input), Ok(253104));
        assert_eq!(solve_part2(input), Ok(499));
    }
}
