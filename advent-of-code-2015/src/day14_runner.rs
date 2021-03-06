#[allow(unused_imports)] // parse_input is used by the aoc macro
use crate::day14::{parse_input, Reindeer};
use aoc_runner_derive::aoc;

enum ReindeerState {
    Flying,
    Resting,
}

struct ReindeerRunner {
    speed: u32,         // in km/s
    fly_duration: u32,  // in s
    rest_duration: u32, // in s
    state: ReindeerState,
    seconds_in_state: u32,
    distance_travelled: u32,
    points: u32,
}

impl ReindeerRunner {
    // Perform a step (= one second) and return the current distance travelled.
    fn step(&mut self) -> u32 {
        self.distance_travelled = match self.state {
            ReindeerState::Flying => self.distance_travelled + self.speed,
            ReindeerState::Resting => self.distance_travelled,
        };

        self.seconds_in_state += 1;

        match self.state {
            ReindeerState::Flying if self.seconds_in_state == self.fly_duration => {
                self.state = ReindeerState::Resting;
                self.seconds_in_state = 0;
            }
            ReindeerState::Resting if self.seconds_in_state == self.rest_duration => {
                self.state = ReindeerState::Flying;
                self.seconds_in_state = 0;
            }
            _ => {}
        }

        self.distance_travelled
    }

    fn add_point(&mut self) {
        self.points += 1;
    }
}

impl From<&Reindeer> for ReindeerRunner {
    fn from(r: &Reindeer) -> Self {
        Self {
            speed: r.speed,
            fly_duration: r.fly_duration,
            rest_duration: r.rest_duration,
            state: ReindeerState::Flying,
            seconds_in_state: 0,
            distance_travelled: 0,
            points: 0,
        }
    }
}

fn race_with_points_system_runners(reindeer: &[Reindeer], duration: u32) -> Vec<ReindeerRunner> {
    let mut runners: Vec<ReindeerRunner> =
        reindeer.iter().map(|reindeer| reindeer.into()).collect();

    for _ in 1..=duration {
        let lead_distance = runners
            .iter_mut()
            .map(|runner| runner.step())
            .max()
            .unwrap();

        runners
            .iter_mut()
            .filter(|runner| runner.distance_travelled == lead_distance)
            .for_each(|runner| runner.add_point());
    }

    runners
}

#[aoc(day14, part2, runners)]
fn solve_part2_runners(reindeer: &[Reindeer]) -> u32 {
    let points = race_with_points_system_runners(reindeer, 2503);

    points.iter().map(|r| r.points).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day14.txt");

        let reindeer = parse_input(input);

        assert_eq!(solve_part2_runners(&reindeer), 1256);
    }
}
