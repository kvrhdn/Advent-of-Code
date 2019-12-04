mod wire;

use common::{console_utils::Timer, grid::*};
use wire::*;

use std::collections::HashSet;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/3
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part1");

    let wires = parse_input(input)?;
    let intersections = get_intersections(wires)?;

    intersections
        .iter()
        .map(|&p| distance_origin(p))
        .filter(|&distance| distance != 0) // origin is also an intersection of the wires
        .min()
        .ok_or_else(|| "expected the wires to have at least one intersection that is not the origin".into())
}

/// See: https://adventofcode.com/2019/day/3#part2
#[wasm_bindgen]
pub fn part2(_input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part2");

    Err("not implemented yet".into())
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

fn get_intersections(wires: Vec<Wire>) -> Result<Vec<Pos>, &'static str> {
    let position_sets = wires
        .into_iter()
        .map(|wire| HashSet::from_iter(wire.positions.into_iter()))
        .collect::<Vec<HashSet<Pos>>>();

    if position_sets.len() != 2 {
        return Err("expected input to contain exactly two wires");
    }
    let set1 = &position_sets[0];
    let set2 = &position_sets[1];

    Ok(set1.intersection(&set2).cloned().collect())
}
