use crate::grid::{distance_origin, Dir};
use aoc_runner_derive::aoc;

type Pos = crate::grid::Pos<i32>;

enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Action {
    fn parse(action: &str) -> Self {
        let n = action[1..].parse::<i32>().unwrap();

        match &action[0..1] {
            "N" => Action::North(n),
            "S" => Action::South(n),
            "E" => Action::East(n),
            "W" => Action::West(n),
            "L" => Action::Left(n),
            "R" => Action::Right(n),
            "F" => Action::Forward(n),
            _ => panic!("Unknown action {}", action),
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Action> + '_ {
    input.lines().map(|line| Action::parse(line))
}

#[aoc(day12, part1)]
fn solve_part1(input: &str) -> i32 {
    let (ship, _) = parse_input(input).fold(
        (Pos::at(0, 0), Dir::Right),
        |(ship, mut dir), action| match action {
            Action::North(n) => (ship.stepn(Dir::Up, n), dir),
            Action::South(n) => (ship.stepn(Dir::Down, n), dir),
            Action::East(n) => (ship.stepn(Dir::Right, n), dir),
            Action::West(n) => (ship.stepn(Dir::Left, n), dir),
            Action::Left(angle) => {
                // angles are always multiples of 90
                for _ in 0..angle / 90 {
                    dir = dir.turn_left();
                }
                (ship, dir)
            }
            Action::Right(angle) => {
                // angles are always multiples of 90
                for _ in 0..angle / 90 {
                    dir = dir.turn_right();
                }
                (ship, dir)
            }
            Action::Forward(n) => (ship.stepn(dir, n), dir),
        },
    );

    distance_origin(&ship)
}

#[aoc(day12, part2)]
fn solve_part2(input: &str) -> i32 {
    let (ship, _) = parse_input(input).fold(
        (Pos::at(0, 0), Pos::at(10, 1)),
        |(mut ship, mut waypoint), action| match action {
            Action::North(n) => (ship, waypoint.stepn(Dir::Up, n)),
            Action::South(n) => (ship, waypoint.stepn(Dir::Down, n)),
            Action::East(n) => (ship, waypoint.stepn(Dir::Right, n)),
            Action::West(n) => (ship, waypoint.stepn(Dir::Left, n)),
            Action::Left(angle) => {
                // angles are always multiples of 90
                for _ in 0..angle / 90 {
                    waypoint = Pos::at(-waypoint.y, waypoint.x);
                }
                (ship, waypoint)
            }
            Action::Right(angle) => {
                // angles are always multiples of 90
                for _ in 0..angle / 90 {
                    waypoint = Pos::at(waypoint.y, -waypoint.x);
                }
                (ship, waypoint)
            }
            Action::Forward(n) => {
                for _ in 0..n {
                    ship += waypoint;
                }
                (ship, waypoint)
            }
        },
    );

    distance_origin(&ship)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
F10
N3
F7
R90
F11";

        assert_eq!(solve_part1(input), 25);
        assert_eq!(solve_part2(input), 286);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day12.txt");

        assert_eq!(solve_part1(input), 1710);
        assert_eq!(solve_part2(input), 62045);
    }
}
