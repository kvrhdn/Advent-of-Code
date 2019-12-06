use common::console_utils::Timer;
use intcode::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/5
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part1");

    let mut program = load_program(input)?;
    let mut computer = Computer::new(&mut program);

    computer.run_with_input(1)?;

    let output = computer.get_output();

    if output.is_empty() {
        return Err("diagnostics test created no output".into());
    }

    if output[0..output.len() - 1]
            .iter()
            .any(|&val| val != 0) {
        return Err("diagnostics test failed".into());
    }

    Ok(*computer.get_output().last().unwrap())
}

/// See: https://adventofcode.com/2019/day/5#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part2");

    let mut program = load_program(input)?;
    let mut computer = Computer::new(&mut program);

    computer.run_with_input(5)?;

    let diagnostic_code = computer.get_output().get(0)
        .ok_or("diagnostics test created no output")?;

    Ok(*diagnostic_code)
}
