use common::console_utils::Timer;
use common::grid::*;
use intcode::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/13
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part1");

    let mut computer = Computer::new_from_input(input)?;

    let mut screen = SparseGrid::<Tile>::new();

    computer.run()?;

    computer
        .get_output_buffer()
        .chunks_exact(3)
        .for_each(|instruction| {
            let x = instruction[0] as i32;
            let y = instruction[1] as i32;
            let tile = FromPrimitive::from_i64(instruction[2]).unwrap();

            screen.set((x, y).into(), tile);
        });

    Ok(screen.iterate().filter(|&tile| *tile == Tile::Block).count() as u32)
}

/// See: https://adventofcode.com/2019/day/13#part2
#[wasm_bindgen]
pub fn part2(_input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part2");

    Ok(0)
}

#[derive(Eq, FromPrimitive, PartialEq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

struct SparseGrid<T> {
    map: HashMap<Pos, T>,
}

impl<T> SparseGrid<T> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn get(&self, pos: Pos) -> Option<&T> {
        self.map.get(&pos)
    }

    fn set(&mut self, pos: Pos, value: T) {
        self.map.insert(pos, value);
    }

    /// Iterates over all filled-in positions, does not visit the positions
    /// that were not set.
    fn iterate<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        self.map.values()
    }
}
