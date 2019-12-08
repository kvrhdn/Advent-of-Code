use common::console_utils::Timer;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static WIDTH: u32 = 25;
static HEIGHT: u32 = 6;

/// See: https://adventofcode.com/2019/day/8
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part1");

    let layers = split_layers(input, WIDTH, HEIGHT);

    let layer_least_0 = layers
        .iter()
        .min_by_key(|&layer| count_char(layer, '0'))
        .ok_or("could not find any layer")?;

    Ok(count_char(layer_least_0, '1') * count_char(layer_least_0, '2'))
}

fn split_layers<'a>(input: &'a str, width: u32, height: u32) -> Vec<&'a [u8]> {
    input
        .trim_end()
        .as_bytes()
        .chunks((width * height) as usize)
        .collect()
}

fn count_char(input: &[u8], c: char) -> u32 {
    bytecount::count(input, c as u8) as u32
}

/// See: https://adventofcode.com/2019/day/8#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part2");

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_layers() {
        assert_eq!(
            split_layers("123456789012", 3, 2),
            vec![
                &[b'1', b'2', b'3', b'4', b'5', b'6'],
                &[b'7', b'8', b'9', b'0', b'1', b'2'],
            ],
        );
    }

    #[test]
    fn test_count_char() {
        assert_eq!(count_char(&[b'1', b'2', b'3', b'1', b'2', b'2'], '0'), 0);
        assert_eq!(count_char(&[b'1', b'2', b'3', b'1', b'2', b'2'], '1'), 2);
        assert_eq!(count_char(&[b'1', b'2', b'3', b'1', b'2', b'2'], '2'), 3);
        assert_eq!(count_char(&[b'1', b'2', b'3', b'1', b'2', b'2'], '3'), 1);
    }
}