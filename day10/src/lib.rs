use common::console_utils::Timer;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/10
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part1");

    Ok(0)
}

/// See: https://adventofcode.com/2019/day/10#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part2");

    Ok(0)
}
