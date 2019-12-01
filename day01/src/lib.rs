use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn part1(_input: String) -> String {
    "TODO: solution of day 01, part 1".to_string()
}

#[wasm_bindgen]
pub fn part2(_input: String) -> String {
    "TODO: solution of day 01, part 2".to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let result = part1("".to_string());

        assert_eq!(result, "Solution of day 01, part 1".to_string());
    }

    #[test]
    fn test_part2() {
        let result = part2("".to_string());

        assert_eq!(result, "Solution of day 01, part 2".to_string());
    }
}
