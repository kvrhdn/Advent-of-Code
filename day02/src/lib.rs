use intcode::IntcodeProgram;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/2
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    let mut program: IntcodeProgram = parse_input(input)?.into();

    program.set_noun_and_verb(12, 2);
    program.run()?;

    Ok(program.get(0))
}

/// See: https://adventofcode.com/2019/day/2#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<u32, JsValue> {
    let program: IntcodeProgram = parse_input(input)?.into();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut test_program = program.clone();

            test_program.set_noun_and_verb(noun, verb);
            test_program.run()?;

            if test_program.get(0) == 19_690_720 {
                return Ok((100 * noun) + verb)
            }
        }
    }

    Err("could not find a noun and verb".into())
}

/// Parse the input as a list of integers.
fn parse_input(input: &str) -> Result<Vec<u32>, &'static str> {
    input
        .trim_end()
        .split(',')
        .map(|l| {
            println!("{}", l);
            l.parse::<u32>()
                .map_err(|_| "could not parse input as integers")
        })
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_input() {
        assert_eq!(crate::parse_input("123,25,0\n"), Ok(vec![123, 25, 0]));
        assert!(crate::parse_input("123,-5,0\n").is_err());
        assert!(crate::parse_input("123,a,0\n").is_err());
    }
}
