use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn part1(input: String) -> i32 {
    input
        .lines()
        .map(|l| l.parse::<i32>().expect("Expected input to only contain integers"))
        .map(fuel_required)
        .sum()
}

#[wasm_bindgen]
pub fn part2(input: String) -> i32 {
    input
        .lines()
        .map(|l| l.parse::<i32>().expect("Expected input to only contain integers"))
        .map(total_fuel_required)
        .sum()
}

/// Fuel required to carry the given mass.
fn fuel_required(mass: i32) -> i32 {
    // integer dision already rounds down
    (mass / 3) - 2
}

/// Total fuel required to carry the given mass, including the mass of the fuel needed.
fn total_fuel_required(mass: i32) -> i32 {
    let fuel = fuel_required(mass);
    if fuel <= 0 {
        return 0
    }

    fuel + total_fuel_required(fuel)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn table_test_fuel_required() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100_756), 33_583);
    }

    #[test]
    fn table_test_total_fuel_required() {
        assert_eq!(total_fuel_required(14), 2);
        assert_eq!(total_fuel_required(1969), 966);
        assert_eq!(total_fuel_required(100_756), 50_346);
    }
}
