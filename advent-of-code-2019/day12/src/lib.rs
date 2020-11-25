mod universe;

use common::console_utils::Timer;
use universe::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/12
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part1");

    let mut universe = parse_input(input)?;

    universe.step(1000);

    Ok(universe.total_energy())
}

/// See: https://adventofcode.com/2019/day/12#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<u64, JsValue> {
    Timer::new("rust::part2");

    let mut universe = parse_input(input)?;

    Ok(universe.find_cycle_time())
}

fn parse_input(input: &str) -> Result<Universe, &'static str> {
    let positions_result: Result<Vec<_>, _> = input.trim_end().lines().map(parse_line).collect();
    let positions = positions_result?;

    if positions.len() != 4 {
        return Err("this implementation expects exactly 4 moons");
    }

    Ok(Universe::new(&positions))
}

fn parse_line(line: &str) -> Result<(i32, i32, i32), &'static str> {
    // example input: <x=-3, y=-2, z=-4>

    let index_x = 2usize + line.find("x=").ok_or("could not parse input")?;
    let index_comma = line.find(", y=").ok_or("could not parse input")?;

    let x = line[index_x..index_comma]
        .parse::<i32>()
        .map_err(|_| "could not parse value of x")?;

    let index_y = 2usize + line.find("y=").ok_or("could not parse input")?;
    let index_comma = line.find(", z=").ok_or("could not parse input")?;

    let y = line[index_y..index_comma]
        .parse::<i32>()
        .map_err(|_| "could not parse value of y")?;

    let index_z = 2usize + line.find("z=").ok_or("could not parse input")?;
    let index_comma = line.find('>').ok_or("could not parse input")?;

    let z = line[index_z..index_comma]
        .parse::<i32>()
        .map_err(|_| "could not parse value of z")?;

    Ok((x, y, z))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let test_cases = vec![
            ("<x=-1, y=0, z=2>", (-1, 0, 2)),
            ("<x=2, y=-10, z=-7>", (2, -10, -7)),
            ("<x=4, y=-8, z=8>", (4, -8, 8)),
            ("<x=3, y=5, z=-1>", (3, 5, -1)),
        ];

        for (input, expected) in test_cases {
            let got = parse_line(input).unwrap();

            assert_eq!(got, expected);
        }
    }
}
