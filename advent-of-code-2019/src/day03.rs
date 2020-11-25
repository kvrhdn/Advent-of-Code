use crate::grid::*;
use aoc_runner_derive::aoc;
use std::collections::HashSet;
use std::iter::FromIterator;
use wire::*;

mod wire;

/// See: https://adventofcode.com/2019/day/3
#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> Result<u32, &'static str> {
    let wires = parse_input(input)?;

    get_intersections(&wires)?
        .iter()
        .filter(|&&&p| p != Pos::at(0, 0)) // origin is also an intersection of the wires
        .map(|&&p| distance_origin(p))
        .min()
        .ok_or_else(|| {
            "expected the wires to have at least one intersection that is not the origin".into()
        })
}

/// See: https://adventofcode.com/2019/day/3#part2
#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> Result<u32, &'static str> {
    let wires = parse_input(input)?;

    get_intersections(&wires)?
        .iter()
        .filter(|&&&p| p != Pos::at(0, 0)) // origin is also an intersection of the wires
        .map(|&&p| {
            wires
                .iter()
                .map(|wire| wire.shortest_length(p).unwrap_or(std::u32::MAX))
                .sum()
        })
        .min()
        .ok_or_else(|| {
            "expected the wires to have at least one intersection that is not the origin".into()
        })
}

fn parse_input(input: &str) -> Result<Vec<Wire>, &'static str> {
    input
        .lines()
        .map(|line| -> Result<Wire, &'static str> {
            let segments = line
                .split(',')
                .map(WireSegment::parse)
                .collect::<Result<Vec<WireSegment>, &'static str>>()?;

            Ok(Wire::from_segments(segments))
        })
        .collect()
}

fn get_intersections(wires: &[Wire]) -> Result<Vec<&Pos>, &'static str> {
    let position_sets = wires
        .iter()
        .map(|wire| HashSet::from_iter(wire.positions.iter()))
        .collect::<Vec<HashSet<&Pos>>>();

    if position_sets.len() != 2 {
        return Err("expected input to contain exactly two wires");
    }
    let set1 = &position_sets[0];
    let set2 = &position_sets[1];

    Ok(set1.intersection(&set2).cloned().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day3.txt");

        assert_eq!(solve_part1(input), Ok(489));
        assert_eq!(solve_part2(input), Ok(93654));
    }
}
