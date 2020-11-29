use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;
use std::cmp;

#[aoc(day12, part1)]
fn solve_part1(input: &str) -> i32 {
    let mut universe = parse_input(input);

    universe.step(1000);

    universe.total_energy()
}

#[aoc(day12, part2)]
fn solve_part2(input: &str) -> u64 {
    let mut universe = parse_input(input);

    universe.find_cycle_time()
}

fn parse_input(input: &str) -> Universe {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    }

    let positions = RE
        .captures_iter(input)
        .map(|cap| {
            let x = cap.get(1).unwrap().as_str().parse().unwrap();
            let y = cap.get(2).unwrap().as_str().parse().unwrap();
            let z = cap.get(3).unwrap().as_str().parse().unwrap();

            (x, y, z)
        })
        .collect::<Vec<_>>();

    Universe::new(&positions)
}

struct Universe {
    // Since the way the gravity is applied is dimension specific (dimensions
    // don't influence each other), we destructe moons into their dimensions
    // and work on the destructed dimensions seperately.
    dimensions: [OneDimensionalUniverse; 3],
}

impl Universe {
    fn new(moons: &[(i32, i32, i32)]) -> Self {
        let dim_x = OneDimensionalUniverse::new([moons[0].0, moons[1].0, moons[2].0, moons[3].0]);
        let dim_y = OneDimensionalUniverse::new([moons[0].1, moons[1].1, moons[2].1, moons[3].1]);
        let dim_z = OneDimensionalUniverse::new([moons[0].2, moons[1].2, moons[2].2, moons[3].2]);

        Self {
            dimensions: [dim_x, dim_y, dim_z],
        }
    }

    fn step(&mut self, delta: u32) {
        self.dimensions.iter_mut().for_each(|d| d.step(delta));
    }

    fn total_energy(&self) -> i32 {
        let mut total_energy = 0;

        for i in 0..4 {
            let (kinetic_energy, potential_energy) = self
                .dimensions
                .iter()
                .map(|d| (d.bodies[i].abs(), d.velocities[i].abs()))
                .fold((0, 0), |acc, (pos, vel)| (acc.0 + pos, acc.1 + vel));

            total_energy += kinetic_energy * potential_energy;
        }

        total_energy
    }

    fn find_cycle_time(&mut self) -> u64 {
        self.dimensions
            .iter_mut()
            .map(|d| d.find_cycle_time())
            .fold(1, lcm)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct OneDimensionalUniverse {
    bodies: [i32; 4],
    velocities: [i32; 4],
}

impl OneDimensionalUniverse {
    fn new(bodies: [i32; 4]) -> Self {
        Self {
            bodies,
            velocities: [0; 4],
        }
    }

    fn step(&mut self, delta: u32) {
        for _ in 0..delta {
            for i in 0..self.bodies.len() {
                for j in 0..self.bodies.len() {
                    if i == j {
                        continue;
                    }

                    self.velocities[i] += match self.bodies[i].cmp(&self.bodies[j]) {
                        cmp::Ordering::Less => 1,
                        cmp::Ordering::Equal => 0,
                        cmp::Ordering::Greater => -1,
                    };
                }
            }

            for i in 0..self.bodies.len() {
                self.bodies[i] += self.velocities[i];
            }
        }
    }

    /// Find the amount of steps needed to return to the same state.
    fn find_cycle_time(&mut self) -> u64 {
        let mut steps = 0;
        let original_state = self.clone();

        loop {
            self.step(1);
            steps += 1;

            if *self == original_state {
                return steps;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part2() {
        let input = r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#;

        assert_eq!(solve_part2(input), 2772);

        let input = r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#;

        assert_eq!(solve_part2(input), 4686774924);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day12.txt");

        assert_eq!(solve_part1(input), 10055);
        assert_eq!(solve_part2(input), 374307970285176);
    }
}
