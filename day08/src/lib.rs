use common::console_utils::Timer;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

/// See: https://adventofcode.com/2019/day/8
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part1");

    let layers = as_layers(input, WIDTH * HEIGHT);

    let layer_least_0 = layers
        .iter()
        .min_by_key(|&layer| bytecount::count(layer, b'0'))
        .ok_or("expected their to be at least one layer")?;

    let checksum = bytecount::count(layer_least_0, b'1') * bytecount::count(layer_least_0, b'2');

    Ok(checksum as u32)
}

/// See: https://adventofcode.com/2019/day/8#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<JsValue, JsValue> {
    Timer::new("rust::part2");

    let layers = as_layers(input, WIDTH * HEIGHT);
    let image = flatten_layers(layers, WIDTH * HEIGHT)?;

    let string = image
        .chunks_exact(WIDTH)
        .map(collect_to_string)
        .collect::<Vec<_>>()
        .join("\n");

    console::log_1(&"The solution to day 08, part 2:".into());
    console::log_1(&string.into());

    Ok("See the console".into())
}

fn as_layers<'a>(input: &'a str, layer_size: usize) -> Vec<&'a [u8]> {
    input
        .trim_end()
        .as_bytes()
        .chunks_exact(layer_size)
        .collect()
}

fn flatten_layers(layers: Vec<&[u8]>, layer_size: usize) -> Result<Vec<char>, &'static str> {
    let mut image = vec![' '; layer_size];

    for i in 0..layer_size {
        let pixel: u8 = layers
            .iter()
            .map(|layer| layer[i])
            .filter(|&pixel| pixel != b'2') // 2 is transparent
            .nth(0)
            .ok_or("did find not any non-transparant pixel")?;

        image[i] = match pixel {
            b'0' => ' ',
            b'1' => '█',
            _ => return Err("expected all pixels to be '0' or '1'".into()),
        };
    }

    Ok(image)
}

fn collect_to_string(chars: &[char]) -> String {
    chars.iter().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_layers() {
        assert_eq!(
            as_layers("123456789012", 6),
            vec![
                &[b'1', b'2', b'3', b'4', b'5', b'6'],
                &[b'7', b'8', b'9', b'0', b'1', b'2'],
            ],
        );
    }
}
