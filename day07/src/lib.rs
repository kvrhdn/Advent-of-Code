use common::console_utils::Timer;
use intcode::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/7
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part1");

    let amplifier_controller_software = load_program(input)?;

    let mut largest_output = 0;

    for ps1 in 0..=4 {
        for ps2 in 0..=4 {
            for ps3 in 0..=4 {
                for ps4 in 0..=4 {
                    for ps5 in 0..=4 {
                        let settings = vec![ps1, ps2, ps3, ps4, ps5];

                        // check that we have 5 unique phase settings
                        if HashSet::<i32>::from_iter(settings.iter().cloned()).len() != 5 {
                            continue;
                        }

                        let output = run_through_amplifiers(&amplifier_controller_software, &settings[0..5])?;

                        if output > largest_output {
                            largest_output = output;
                        }
                    }
                }
            }
        }
    }

    Ok(largest_output)
}

/// See: https://adventofcode.com/2019/day/7#part2
#[wasm_bindgen]
pub fn part2(_input: &str) -> Result<u32, JsValue> {
    Timer::new("rust::part2");

    Ok(0)
}

fn run_through_amplifiers(amplifier_controller_software: &[i32], phase_settings: &[i32]) -> Result<i32, &'static str> {
    let mut runtime = vec![0; amplifier_controller_software.len()];

    let mut input = 0;

    for phase_setting in phase_settings {
        runtime.copy_from_slice(amplifier_controller_software);
        let mut amp = Computer::new(&mut runtime);

        amp.run_with_input(&[*phase_setting, input])?;

        input = *amp
            .get_output()
            .first()
            .ok_or("amplifier controller software did not output any value")?;
    }

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_through_amplifiers() {
        // (program, phase settings, expected output)
        let test_cases = vec![
            (
                vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                vec![4, 3, 2, 1, 0],
                43210,
            ),
            (
                vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0],
                vec![0, 1, 2, 3, 4],
                54321,
            ),
            (
                vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0],
                vec![1, 0, 4, 3, 2],
                65210,
            ),
        ];

        for (program, phase_settings, expected) in test_cases {
            let got = run_through_amplifiers(&program, &phase_settings).unwrap();

            if got != expected {
                panic!("run_through_amplifiers({:?}, {:?}) = {}, but expected {}", program, phase_settings, got, expected);
            }
        }
    }
}
