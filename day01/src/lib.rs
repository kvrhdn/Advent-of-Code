use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn part1(input: String) -> u32 {
    input.lines()
        .map(|l| l.parse::<u32>().expect("aaa"))
        .map(fuel_required)
        .sum()
}

#[wasm_bindgen]
pub fn part2(_input: String) -> String {
    "TODO: solution of day 01, part 2".to_string()
}

fn fuel_required(mass: u32) -> u32 {
    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn table_test_fuel_required() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100_756), 33583);
    }
}
