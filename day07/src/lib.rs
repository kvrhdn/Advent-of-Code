use common::console_utils::Timer;
use intcode::*;
use itertools::Itertools;
use std::ops::RangeInclusive;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/7
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part1");

    let program = load_program(input)?;

    find_largest_output(&program, 0..=4, amplify_single_passthrough)
        .map_err(|e| e.into())
}

/// See: https://adventofcode.com/2019/day/7#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part2");

    let program = load_program(input)?;

    find_largest_output(&program, 5..=9, amplify_with_feedback)
        .map_err(|e| e.into())
}

fn find_largest_output<F>(
    program: &[i32],
    phase_range: RangeInclusive<i32>,
    run: F,
) -> Result<i32, &'static str>
where
    F: Fn(&[i32], &[i32]) -> Result<i32, &'static str>,
{
    let mut largest_output = 0;

    let mut phase_settings: Vec<i32> = phase_range.collect();
    let permutations = permutohedron::Heap::new(&mut phase_settings);

    for phase_settings in permutations {
        let output = run(&program, &phase_settings)?;

        if output > largest_output {
            largest_output = output;
        }
    }

    Ok(largest_output)
}

fn amplify_single_passthrough(
    amplifier_controller_software: &[i32],
    phase_settings: &[i32],
) -> Result<i32, &'static str> {

    let mut program_copy = vec![0; amplifier_controller_software.len()];

    let mut signal = 0;

    for phase in phase_settings {
        program_copy.copy_from_slice(amplifier_controller_software);
        let mut amp = Computer::new(&mut program_copy);

        amp.put_input(*phase);
        amp.put_input(signal);
        amp.run()?;

        signal = amp.get_output()
            .ok_or("amplifier controller software did not output a value")?;
    }

    Ok(signal)
}

fn amplify_with_feedback(
    amplifier_controller_software: &[i32],
    phase_settings: &[i32],
) -> Result<i32, &'static str> {
    // allocate memory for amps
    let mut memory_buffers = vec![0; phase_settings.len() * amplifier_controller_software.len()];
    
    // initialize memory with control software and link to an amp
    let mut amps: Vec<Computer> = memory_buffers
        .chunks_exact_mut(amplifier_controller_software.len())
        .map(|buffer| {
            buffer.copy_from_slice(amplifier_controller_software);
            Computer::new(buffer)
        })
        .collect();

    // load phase in each amp
    amps.iter_mut()
        .zip_eq(phase_settings)
        .for_each(|(computer, phase)| computer.put_input(*phase));

    let mut amp_index = 0;
    let mut signal = 0;

    // let the amps amplify the signal until all amps are halted
    loop {
        let amp = amps.get_mut(amp_index).unwrap();

        // Stop looping if the next amp has halted, this isn't exactly what the
        // puzzle describes but it in practice this will always stop when amp 1
        // has halted and thus all other amps as well.
        if amp.get_state() == State::HALTED {
            break;
        }

        amp.put_input(signal);
        amp.run()?;

        signal = amp.get_output()
            .ok_or("amplifier controller software did not output a value")?;

        amp_index = (amp_index + 1) % amps.len();
    }

    Ok(signal)
}
