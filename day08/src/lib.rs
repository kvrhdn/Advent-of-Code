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

    let layers = split_layers(input, WIDTH, HEIGHT);

    let layer_least_0 = layers
        .iter()
        .min_by_key(|&layer| count_chars(layer, '0'))
        .ok_or("expected their to be at least one layer")?;

    Ok(count_chars(layer_least_0, '1') * count_chars(layer_least_0, '2'))
}

/// See: https://adventofcode.com/2019/day/8#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<JsValue, JsValue> {
    Timer::new("rust::part2");

    let layers = split_layers(input, WIDTH, HEIGHT);

    let mut image = [0u8; WIDTH * HEIGHT];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let pixel_index = (y * WIDTH) + x;

            let pixel: u8 = layers
                .iter()
                .map(|layer| layer[pixel_index])
                .filter(|&pixel| pixel != b'2') // 2 is transparent
                .nth(0)
                .ok_or("did find not any non-transparant pixel")?;

            image[pixel_index] = pixel;
        }
    }

    let s: Vec<String> = image
        .chunks(WIDTH)
        .map(|row| {
            row.iter()
                .map(|pixel| match pixel {
                    b'0' => ' ',
                    b'1' => 'X',
                    _ => panic!("expected all pixels to be '0' or '1'"),
                })
                .collect()
        })
        .collect();

    console::log_1(&"The solution to day 08, part 2:".into());
    console::log_1(&s.join("\n").into());

    Ok("See the console".into())
}

fn split_layers<'a>(input: &'a str, width: usize, height: usize) -> Vec<&'a [u8]> {
    input.trim_end().as_bytes().chunks(width * height).collect()
}

fn count_chars(input: &[u8], c: char) -> u32 {
    bytecount::count(input, c as u8) as u32
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
        assert_eq!(count_chars(&[b'1', b'2', b'3', b'1', b'2', b'2'], '0'), 0);
        assert_eq!(count_chars(&[b'1', b'2', b'3', b'1', b'2', b'2'], '1'), 2);
        assert_eq!(count_chars(&[b'1', b'2', b'3', b'1', b'2', b'2'], '2'), 3);
        assert_eq!(count_chars(&[b'1', b'2', b'3', b'1', b'2', b'2'], '3'), 1);
    }
}
