use crate::intcode::*;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::ops::RangeInclusive;

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> i64 {
    let program = load_program(input).unwrap();

    find_largest_output(&program, 0..=4, amplify_single_passthrough)
}

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> i64 {
    let program = load_program(input).unwrap();

    find_largest_output(&program, 5..=9, amplify_with_feedback)
}

fn find_largest_output<F>(program: &[i64], phase_range: RangeInclusive<i64>, run: F) -> i64
where
    F: Fn(&[i64], Vec<i64>) -> i64,
{
    phase_range
        .permutations(5)
        .map(|phase_settings| run(&program, phase_settings))
        .max()
        .unwrap()
}

fn amplify_single_passthrough(
    amplifier_controller_software: &[i64],
    phase_settings: Vec<i64>,
) -> i64 {
    let mut signal = 0;

    for phase in phase_settings {
        let mut amp = Computer::new(amplifier_controller_software.to_owned());

        amp.put_input(phase);
        amp.put_input(signal);
        amp.run().unwrap();

        signal = amp.get_output().unwrap();
    }

    signal
}

fn amplify_with_feedback(amplifier_controller_software: &[i64], phase_settings: Vec<i64>) -> i64 {
    let mut amps: Vec<Computer> = phase_settings
        .iter()
        .map(|&phase| {
            let mut amp = Computer::new(amplifier_controller_software.to_owned());
            amp.put_input(phase);
            amp
        })
        .collect();

    let mut amp_index = 0;
    let mut signal = 0;

    loop {
        let amp = amps.get_mut(amp_index).unwrap();

        // Stop looping if the next amp has halted, this isn't exactly what the
        // puzzle describes but it in practice this will always stop when amp 1
        // has halted and thus all other amps as well.
        if amp.get_state() == State::HALTED {
            break;
        }

        amp.put_input(signal);
        amp.run().unwrap();

        signal = amp.get_output().unwrap();

        amp_index = (amp_index + 1) % amps.len();
    }

    signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        let examples = vec![
            ("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210),
            ("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", 54321),
            ("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", 65210),
        ];
        for (input, expected) in examples {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn examples_part2() {
        let examples = vec![
            ("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", 139629729),
            ("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", 18216),
        ];
        for (input, expected) in examples {
            assert_eq!(solve_part2(input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day7.txt");

        assert_eq!(solve_part1(input), 116680);
        assert_eq!(solve_part2(input), 89603079);
    }
}
