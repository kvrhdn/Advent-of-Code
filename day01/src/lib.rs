use common::console_utils::Timer;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Calculate the sum of the fuel requirements for all of the modules on the spacecraft.
/// See: https://adventofcode.com/2019/day/1
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part1");

    let sum = parse_input(input)?
        .iter()
        .map(|&mass| fuel_required(mass))
        .sum();

    Ok(sum)
}

/// Calculate the sum of the fuel requirements for all of the modules on the spacecraft, also
/// taking into account the mass of the added fuel.
/// See: https://adventofcode.com/2019/day/1#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part2");

    let sum = parse_input(input)?
        .iter()
        .map(|&mass| total_fuel_required(mass))
        .sum();

    Ok(sum)
}

/// Parse the input (a list of modules their masses) as a list of integers.
fn parse_input(input: &str) -> Result<Vec<i32>, &'static str> {
    input
        .lines()
        .map(|l| {
            l.parse::<i32>()
                .map_err(|_| "could not parse input as integers")
        })
        .collect()
}

/// Fuel required to carry the given mass.
fn fuel_required(mass: i32) -> i32 {
    // integer dision already rounds down
    (mass / 3) - 2
}

/// Total fuel required to carry the given mass, including the mass of the fuel itself.
fn total_fuel_required(mass: i32) -> i32 {
    let fuel = fuel_required(mass);
    if fuel <= 0 {
        return 0;
    }

    fuel + total_fuel_required(fuel)
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_input() {
        assert_eq!(crate::parse_input("123\n-5\n0\n"), Ok(vec![123, -5, 0]));
        assert!(crate::parse_input("123\na\n0\n").is_err());
    }

    #[test]
    fn fuel_required() {
        assert_eq!(crate::fuel_required(12), 2);
        assert_eq!(crate::fuel_required(14), 2);
        assert_eq!(crate::fuel_required(1969), 654);
        assert_eq!(crate::fuel_required(100_756), 33_583);
    }

    #[test]
    fn total_fuel_required() {
        assert_eq!(crate::total_fuel_required(14), 2);
        assert_eq!(crate::total_fuel_required(1969), 966);
        assert_eq!(crate::total_fuel_required(100_756), 50_346);
    }
}
