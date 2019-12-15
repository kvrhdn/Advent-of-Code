use common::console_utils::Timer;
use intcode::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/5
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i64, JsValue> {
    Timer::new("rust::part1");

    let mut computer = Computer::new_from_input(input)?;

    computer.run_diagnostics_test(1)
        .map_err(|err| err.into())
}

/// See: https://adventofcode.com/2019/day/5#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i64, JsValue> {
    Timer::new("rust::part2");

    let mut computer = Computer::new_from_input(input)?;

    computer.run_diagnostics_test(5)
        .map_err(|err| err.into())
}
