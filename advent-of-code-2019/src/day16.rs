use aoc_runner_derive::aoc;
use stutter::StutteringIterator;

mod stutter;

/// See: https://adventofcode.com/2019/day/16
#[aoc(day16, part1)]
pub fn solve_part1(input: &str) -> Result<String, &'static str> {

    let mut digits = parse_input(input)?;
    let pattern = vec![0, 1, 0, -1];

    let result = apply_fft_repeated(&mut digits, &pattern, 100);

    Ok(result
        .iter()
        .take(8)
        .map(|v| v.to_string())
        .collect::<String>()
        .into())
}

/// See: https://adventofcode.com/2019/day/16#part2
#[aoc(day16, part2)]
pub fn solve_part2(_input: &str) -> Result<i32, &'static str> {
    Ok(0)
}

fn parse_input(input: &str) -> Result<Vec<i32>, &'static str> {
    input
        .trim_end()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|v| v as i32)
                .ok_or("could not parase character as integer")
        })
        .collect()
}

/// Applies the Flawed Frequency Transmission algorithm on the given input
/// using the given pattern. The result is stored in output, which is expected
/// to be at least as large as input.
fn apply_fft<'a>(input: &[i32], output: &'a mut [i32], pattern: &[i32]) {
    for i in 0..input.len() {
        let mut pattern_iter = pattern
            .iter()
            .cycle()
            .stutter(i)
            .skip(1);

        let mut acc = 0;

        for value in input.iter() {
            acc += value * pattern_iter.next().unwrap();
        }

        output[i] = last_digit(acc);
    }
}

fn apply_fft_repeated<'a>(mut input: &'a mut [i32], pattern: &[i32], times: u32) -> &'a [i32] {
    // to avoid memory allocations, perform FFT back-and-forth on two pre-allocated buffers
    let mut output = &mut vec![0; input.len()];

    if times % 2 != 0 {
        panic!("the amount of times FFT is applied is always assumed to be even");
    }

    for _ in 0..times / 2 {
        apply_fft(input, &mut output, pattern);
        apply_fft(output, &mut input, pattern);
    }

    input
}

fn last_digit(value: i32) -> i32 {
    value.abs() % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_fft_single() {
        let test_cases = vec![
            (vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 8, 2, 2, 6, 1, 5, 8]),
            (vec![4, 8, 2, 2, 6, 1, 5, 8], vec![3, 4, 0, 4, 0, 4, 3, 8]),
            (vec![3, 4, 0, 4, 0, 4, 3, 8], vec![0, 3, 4, 1, 5, 5, 1, 8]),
            (vec![0, 3, 4, 1, 5, 5, 1, 8], vec![0, 1, 0, 2, 9, 4, 9, 8]),
        ];
        let pattern = vec![0, 1, 0, -1];

        let mut output = vec![0; 8];

        for (input, expected) in test_cases {
            apply_fft(&input, &mut output, &pattern);

            if output != expected {
                panic!(
                    "\napply_fft on {:?} with pattern {:?}\ngot:     {:?}\nexpected {:?}\n",
                    input, pattern, output, expected
                );
            }
        }
    }

    #[test]
    fn test_apply_fft_repeated() {
        let test_cases = vec![
            (
                vec![8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4, 5, 5, 9, 5],
                vec![2, 4, 1, 7, 6, 1, 7, 6],
            ),
            (
                vec![1, 9, 6, 1, 7, 8, 0, 4, 2, 0, 7, 2, 0, 2, 2, 0, 9, 1, 4, 4, 9, 1, 6, 0, 4, 4, 1, 8, 9, 9, 1, 7],
                vec![7, 3, 7, 4, 5, 4, 1, 8],
            ),
            (
                vec![6, 9, 3, 1, 7, 1, 6, 3, 4, 9, 2, 9, 4, 8, 6, 0, 6, 3, 3, 5, 9, 9, 5, 9, 2, 4, 3, 1, 9, 8, 7, 3],
                vec![5, 2, 4, 3, 2, 1, 3, 3],
            ),
        ];

        let pattern = vec![0, 1, 0, -1];

        for (mut input, expected) in test_cases {
            let original_input = input.clone();

            let got = apply_fft_repeated(&mut input, &pattern, 100);

            if expected != &got[..8] {
                panic!("apply_fft 100x on {:?} with pattern {:?}, got (first 8 digits only): {:?}, expected {:?}", original_input, pattern, &got[..8], expected);
            }
        }
    }

    #[test]
    fn test_last_digit() {
        assert_eq!(last_digit(38), 8);
        assert_eq!(last_digit(-17), 7);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day16.txt");

        assert_eq!(solve_part1(input), Ok("11833188".to_owned()));
        assert_eq!(solve_part2(input), Ok(0));
    }
}
