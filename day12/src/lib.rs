mod moon;
mod vec3;

use common::console_utils::Timer;
use moon::*;
use vec3::Vec3;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/12
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part1");

    let moons = parse_input(input)?
        .iter()
        .map(|&pos| Moon::new(pos))
        .collect::<Vec<Moon>>();

    let mut universe = Universe::new(moons);

    universe.step(1000);

    Ok(universe.total_energy())
}

/// See: https://adventofcode.com/2019/day/12#part2
#[wasm_bindgen]
pub fn part2(_input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part2");

    Ok(0)
}

fn parse_input(input: &str) -> Result<Vec<Vec3>, &'static str> {
    input.trim_end().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Vec3, &'static str> {
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

    Ok(Vec3 { x, y, z })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
        let expected = vec![
            Vec3 { x: -1, y: 0, z: 2 },
            Vec3 { x: 2, y: -10, z: -7 },
            Vec3 { x: 4, y: -8, z: 8 },
            Vec3 { x: 3, y: 5, z: -1 },
        ];

        assert_eq!(parse_input(input), Ok(expected));
    }
}
