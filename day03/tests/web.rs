#![cfg(target_arch = "wasm32")]

use day03::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

static INPUT: &str = include_str!("../input.txt");

#[wasm_bindgen_test]
fn day01_part1() {
    assert!(part1(INPUT).is_err());
}

#[wasm_bindgen_test]
fn day01_part2() {
    assert!(part2(INPUT).is_err());
}
