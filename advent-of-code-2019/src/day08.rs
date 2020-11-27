use aoc_runner_derive::aoc;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc(day8, part1)]
fn solve_part1(input: &str) -> usize {
    let layers = as_layers(input, WIDTH * HEIGHT);

    let layer_least_0 = layers
        .iter()
        .min_by_key(|&layer| bytecount::count(layer, b'0'))
        .unwrap();

    bytecount::count(layer_least_0, b'1') * bytecount::count(layer_least_0, b'2')
}

#[aoc(day8, part2)]
fn solve_part2(input: &str) -> String {
    let layers = as_layers(input, WIDTH * HEIGHT);
    let image = flatten_layers(layers, WIDTH * HEIGHT);

    image
        .chunks_exact(WIDTH)
        .map(collect_to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

fn as_layers(input: &str, layer_size: usize) -> Vec<&[u8]> {
    input
        .trim_end()
        .as_bytes()
        .chunks_exact(layer_size)
        .collect()
}

fn flatten_layers(layers: Vec<&[u8]>, layer_size: usize) -> Vec<char> {
    let mut image = vec![' '; layer_size];

    for i in 0..layer_size {
        let pixel: u8 = layers
            .iter()
            .map(|layer| layer[i])
            .find(|&pixel| pixel != b'2') // 2 is transparent
            .unwrap();

        image[i] = match pixel {
            b'0' => ' ',
            b'1' => '█',
            _ => panic!("expected all pixels to be '0' or '1'"),
        };
    }

    image
}

fn collect_to_string(chars: &[char]) -> String {
    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day8.txt");

        assert_eq!(solve_part1(input), 2048);
        assert_eq!(
            solve_part2(input),
            r#"█  █ ████ █   █ ██  █  █ 
█  █ █    █   ██  █ █ █  
████ ███   █ █ █  █ ██   
█  █ █      █  ████ █ █  
█  █ █      █  █  █ █ █  
█  █ █      █  █  █ █  █ "#
        );
    }
}
