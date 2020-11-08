use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn origin() -> Self {
        Position { x: 0, y: 0 }
    }

    fn move_direction(&mut self, direction: char) {
        match direction {
            '>' => self.x += 1,
            '<' => self.x -= 1,
            '^' => self.y += 1,
            'v' => self.y -= 1,
            _ => panic!(format!("Unexpected movement instruction {}", direction)),
        }
    }
}

struct InfiniteGridOfHouses {
    map: HashMap<Position, u32>,
}

impl InfiniteGridOfHouses {
    fn new() -> Self {
        InfiniteGridOfHouses {
            map: HashMap::<Position, u32>::new(),
        }
    }

    fn visit(&mut self, pos: Position) {
        let house = self.map.entry(pos).or_default();
        *house += 1;
    }

    fn houses_with_presents(&self) -> usize {
        self.map.values().filter(|&&h| h > 0).count() as usize
    }
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut houses = InfiniteGridOfHouses::new();

    let mut pos = Position::origin();

    houses.visit(pos);

    for m in input.chars() {
        pos.move_direction(m);
        houses.visit(pos);
    }

    houses.houses_with_presents() as u32
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut houses = InfiniteGridOfHouses::new();

    let mut pos_santa = Position::origin();
    let mut pos_robot = Position::origin();

    let mut santas_turn = true;

    houses.visit(pos_santa);
    houses.visit(pos_robot);

    for m in input.chars() {
        let pos = if santas_turn {
            &mut pos_santa
        } else {
            &mut pos_robot
        };

        pos.move_direction(m);
        houses.visit(*pos);

        santas_turn = !santas_turn;
    }

    houses.houses_with_presents() as u32
}

#[cfg(test)]
mod tests {
    use crate::day03::*;

    #[test]
    fn part1_examples() {
        #[rustfmt::skip]
        let examples = vec![
            (">", 2),
            ("^>v<", 4),
            ("^v^v^v^v^v", 2)
        ];

        for e in examples {
            assert_eq!(solve_part1(e.0), e.1);
        }
    }

    #[test]
    fn part2_examples() {
        #[rustfmt::skip]
        let examples = vec![
            ("^v", 3),
            ("^>v<", 3),
            ("^v^v^v^v^v", 11)
        ];

        for e in examples {
            assert_eq!(solve_part2(e.0), e.1);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day3.txt");

        assert_eq!(solve_part1(input), 2592);
        assert_eq!(solve_part2(input), 2360);
    }
}
