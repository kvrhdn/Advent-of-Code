use common::console_utils::Timer;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/9
#[wasm_bindgen]
pub fn part1(_input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part1");

    Ok(0)
}

/// See: https://adventofcode.com/2019/day/9#part2
#[wasm_bindgen]
pub fn part2(_input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part2");

    Ok(0)
}
