#![cfg(target_arch = "wasm32")]

use day07::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

static INPUT: &str = include_str!("../input.txt");

#[wasm_bindgen_test]
fn day01_part1() {
    assert_eq!(part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), Ok(43210));
    assert_eq!(part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"), Ok(54321));
    assert_eq!(part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), Ok(65210));

    assert_eq!(part1(INPUT), Ok(116680));
}

#[wasm_bindgen_test]
fn day01_part2() {
    assert_eq!(part2(INPUT), Ok(0));
}
