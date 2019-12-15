use common::console_utils::Timer;
use intcode::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/9
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i64, JsValue> {
    Timer::new("rust::part1");

    // TODO add function run_diagnostics_test to intcode::Computer, this is same code as day 05
    let mut computer = Computer::new_from_input(input)?;

    computer.put_input(1);
    computer.run()?;

    let output = computer.get_output_buffer();
    if output.is_empty() {
        return Err("diagnostics test created no output".into());
    }

    if output[0..output.len() - 1]
            .iter()
            .any(|&val| val != 0) {
        return Err(format!("diagnostics test failed: {:?}", output).into());
    }

    Ok(*output.last().unwrap())
}

/// See: https://adventofcode.com/2019/day/9#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i64, JsValue> {
    Timer::new("rust::part2");

    let mut computer = Computer::new_from_input(input)?;

    computer.put_input(2);
    computer.run()?;

    computer.get_output()
        .ok_or_else(|| "BOOST program did not output any values".into())
}
