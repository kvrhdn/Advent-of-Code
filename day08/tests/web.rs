#![cfg(target_arch = "wasm32")]

use day08::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

static INPUT: &str = include_str!("../input.txt");

#[wasm_bindgen_test]
fn day08_part1() {
    assert_eq!(part1(INPUT), Ok(2048));
}

#[wasm_bindgen_test]
fn day08_part2() {
    assert_eq!(part2(INPUT), Ok("See the console".into()));
}
