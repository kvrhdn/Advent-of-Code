#![cfg(target_arch = "wasm32")]

use day06::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

static INPUT: &str = include_str!("../input.txt");

#[wasm_bindgen_test]
fn day01_part1() {
    assert_eq!(part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n"), Ok(42));

    assert_eq!(part1(INPUT), Ok(253104));
}

#[wasm_bindgen_test]
fn day01_part2() {
    assert_eq!(part2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"), Ok(4));

    assert_eq!(part2(INPUT), Ok(499));
}
