use common::grid::*;

use std::collections::HashSet;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/3
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    let wires: Vec<Wire> = parse_input(input)?
        .into_iter()
        .map(Wire::from_segments)
        .collect();

    let position_sets: Vec<HashSet<Pos>> = wires
        .into_iter()
        .map(|wire| HashSet::from_iter(wire.positions.into_iter()))
        .collect();

    if position_sets.len() != 2 {
        return Err("expected input to contain exactly two wires".into());
    }
    let set_0 = position_sets.get(0).unwrap();
    let set_1 = position_sets.get(1).unwrap();

    let intersection: Vec<&Pos> = set_0.intersection(set_1).collect();

    intersection
        .iter()
        .map(|&&p| distance_origin(p))
        .filter(|&distance| distance != 0)   // origin is also an intersection of the wires
        .min()
        .ok_or_else(|| "expected the wires to have at least one intersection that is not the origin".into())
}

/// See: https://adventofcode.com/2019/day/3#part2
#[wasm_bindgen]
pub fn part2(_input: &str) -> Result<u32, JsValue> {
    Err("not implemented yet".into())
}

/// A wire is represented as a sequence of the positions that it covers.
struct Wire {
    positions: Vec<Pos>,
}

impl Wire {
    /// Construct a wire from a series of wire segments, starting at position (0, 0).
    fn from_segments(segments: Vec<WireSegment>) -> Self {
        let mut positions = Vec::<Pos>::new();
        let mut curr = Pos::at(0, 0);

        positions.push(curr);

        for s in segments {
            for _ in 0..s.length {
                curr = curr.step(s.direction);

                positions.push(curr);
            }
        }

        Self { positions }
    }
}

/// A wire segment is a straight part of a wire, described relatively to another segment.
#[derive(Debug, Eq, PartialEq)]
struct WireSegment {
    direction: Dir,
    length: u32,
}

fn parse_input(input: &str) -> Result<Vec<Vec<WireSegment>>, &'static str> {
    input.lines().map(parse_as_wire_segments).collect()
}

fn parse_as_wire_segments(input: &str) -> Result<Vec<WireSegment>, &'static str> {
    input.split(',').map(parse_wire_segment).collect()
}

fn parse_wire_segment(segment: &str) -> Result<WireSegment, &'static str> {
    let first_char = segment
        .chars()
        .nth(0)
        .ok_or("could not access the first character")?;

    let direction = match first_char {
        'U' => Dir::Up,
        'D' => Dir::Down,
        'L' => Dir::Left,
        'R' => Dir::Right,
        _ => return Err("invalid direction"),
    };

    let length = segment[1..]
        .parse::<u32>()
        .map_err(|_| "could not parse input as integers")?;

    Ok(WireSegment { direction, length })
}

#[cfg(test)]
mod tests {
    use crate::{Wire, WireSegment};
    use common::grid::{Dir::*, Pos};

    #[test]
    fn parse_input_into_wire_segments() {
        let input = "U123,D32\nL12345,R1";
        let expected = vec![
            vec![WireSegment { direction: Up, length: 123 }, WireSegment { direction: Down, length: 32 }],
            vec![WireSegment { direction: Left, length: 12345 }, WireSegment { direction: Right, length: 1 }],
        ];

        let result = crate::parse_input(&input).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn wire_from_wire_segments() {
        let input = vec![
            WireSegment { direction: Up, length: 2 },
            WireSegment { direction: Left, length: 1 },
            WireSegment { direction: Down, length: 3 },
            WireSegment { direction: Right, length: 1 },
        ];

        let wire = Wire::from_segments(input);

        assert_eq!(wire.positions, vec![
            Pos::at(0, 0),
            Pos::at(0, 1), Pos::at(0, 2),
            Pos::at(-1, 2),
            Pos::at(-1, 1), Pos::at(-1, 0), Pos::at(-1, -1),
            Pos::at(0, -1),
        ]);
    }
}
