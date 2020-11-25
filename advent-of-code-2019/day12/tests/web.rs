#![cfg(target_arch = "wasm32")]

use day12::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

static INPUT: &str = include_str!("../input.txt");

#[wasm_bindgen_test]
fn day12_part1() {
    assert_eq!(part1(INPUT), Ok(10055));
}

#[wasm_bindgen_test]
fn day12_part2() {
    assert_eq!(part2(INPUT), Ok(374307970285176));
}
