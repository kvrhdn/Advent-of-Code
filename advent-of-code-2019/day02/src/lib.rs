use common::console_utils::Timer;
use intcode::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/2
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i64, JsValue> {
    Timer::new("rust::part1");

    let mut computer = Computer::new_from_input(input)?;
    set_noun_and_verb(&mut computer, 12, 2);

    computer.run()?;

    Ok(computer.get(0))
}

/// See: https://adventofcode.com/2019/day/2#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i64, JsValue> {
    Timer::new("rust::part2");

    let program = load_program(input)?;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = Computer::new(program.clone());
            set_noun_and_verb(&mut computer, noun, verb);

            computer.run()?;

            if computer.get(0) == 19_690_720 {
                return Ok((100 * noun) + verb);
            }
        }
    }

    Err("could not find a noun and verb".into())
}

fn set_noun_and_verb(computer: &mut Computer, noun: i64, verb: i64) {
    computer.set(1, noun);
    computer.set(2, verb);
}
