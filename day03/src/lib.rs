use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/3
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    let _wireSegments = parse_input(input)?;

    // trace both entries, get list of positions
    // grab union of both
    // find nearest
    // return

    Err("not implemented yet".into())
}

/// See: https://adventofcode.com/2019/day/3#part2
#[wasm_bindgen]
pub fn part2(_input: &str) -> Result<u32, JsValue> {
    Err("not implemented yet".into())
}

#[derive(Debug, Eq, PartialEq)]
struct WireSegment {
    direction: Direction,
    length: u32,
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("aaa"),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Vec<WireSegment>>, &'static str> {
    input.lines()
        .map(|l| {
            l
                .split(',')
                .map(|segment| {
                    let direction: Direction = segment
                        .chars().nth(0)
                        .ok_or("could not access the first character")?
                        .into();

                    let length = segment[1..].parse::<u32>()
                        .map_err(|_| "could not parse input as integers")?;

                    Ok(WireSegment {
                        direction,
                        length,
                    })
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::Direction::*;
    use crate::WireSegment;

    #[test]
    fn parse_input() {
        let input = "U123,D32\nL12345,R1";
        let expected = vec![
            vec![WireSegment { direction: Up, length: 123 }, WireSegment { direction: Down, length: 32 }],
            vec![WireSegment { direction: Left, length: 12345 }, WireSegment { direction: Right, length: 1 }],
        ];

        let result = crate::parse_input(&input).unwrap();

        assert_eq!(result, expected);
    }
}
