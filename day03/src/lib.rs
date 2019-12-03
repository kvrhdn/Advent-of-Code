use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/3
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    Err("not implemented yet".into())
}

/// See: https://adventofcode.com/2019/day/3#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<u32, JsValue> {
    Err("not implemented yet".into())
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
