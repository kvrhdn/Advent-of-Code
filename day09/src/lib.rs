use common::console_utils::Timer;
use intcode::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/9
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i64, JsValue> {
    Timer::new("rust::part1");

    let mut computer = Computer::new_from_input(input)?;

    computer.run_diagnostics_test(1)
        .map_err(|err| err.into())
}

/// See: https://adventofcode.com/2019/day/9#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i64, JsValue> {
    Timer::new("rust::part2");

    let mut computer = Computer::new_from_input(input)?;

    computer.put_input(2);
    computer.run()?;

    computer
        .get_output()
        .ok_or_else(|| "BOOST program did not output any values".into())
}
