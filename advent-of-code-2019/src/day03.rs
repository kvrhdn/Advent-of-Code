use crate::grid::*;
use aoc_runner_derive::aoc;
use std::collections::HashMap;

fn parse_wire(wire: &str) -> HashMap<Pos, u32> {
    let mut positions = HashMap::new();

    let mut curr = Pos::at(0, 0);
    let mut steps = 0;

    for segment in wire.split(',') {
        let direction = match &segment[0..1] {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("invalid direction"),
        };
        let length = segment[1..].parse().unwrap();

        for _ in 0..length {
            curr = curr.step(direction);
            steps += 1;

            positions.insert(curr, steps);
        }
    }

    positions
}

fn parse_input(input: &str) -> HashMap<Pos, u32> {
    let mut lines = input.lines();

    let mut positions_wire1 = parse_wire(lines.next().unwrap());
    let positions_wire2 = parse_wire(lines.next().unwrap());

    // only retain positions that are also in positions_wire2, simultaneously
    // add up both values
    positions_wire1.retain(|pos, steps| {
        if let Some(other_steps) = positions_wire2.get(pos) {
            *steps += other_steps;
            true
        } else {
            false
        }
    });

    positions_wire1
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> u32 {
    let positions = parse_input(input);

    positions
        .keys()
        .map(|&pos| distance_origin(pos))
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> u32 {
    let positions = parse_input(input);

    *positions.values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        #[rustfmt::skip]
        let examples = vec![
            (
                "R8,U5,L5,D3\nU7,R6,D4,L4",
                6, 30,
            ),
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                159, 610,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135, 410,
            ),
        ];
        for (input, expected_part1, expected_part2) in examples {
            assert_eq!(solve_part1(input), expected_part1);
            assert_eq!(solve_part2(input), expected_part2);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day3.txt");

        assert_eq!(solve_part1(input), 489);
        assert_eq!(solve_part2(input), 93654);
    }
}
