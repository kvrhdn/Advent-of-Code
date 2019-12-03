use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/2
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    let program = parse_input(input)?;

    let mut computer = intcode::Computer::new(program);
    set_noun_and_verb(&mut computer, 12, 2);

    computer.run()?;

    Ok(computer.get(0))
}

/// See: https://adventofcode.com/2019/day/2#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<u32, JsValue> {
    let program = parse_input(input)?;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = intcode::Computer::new(program.clone());

            set_noun_and_verb(&mut computer, noun, verb);

            computer.run()?;

            if computer.get(0) == 19_690_720 {
                return Ok((100 * noun) + verb);
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
            l.parse::<u32>()
                .map_err(|_| "could not parse input as integers")
        })
        .collect()
}

pub fn set_noun_and_verb(computer: &mut intcode::Computer, noun: u32, verb: u32) {
    computer.set(1, noun);
    computer.set(2, verb);
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
