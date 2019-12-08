use common::console_utils::Timer;
use intcode::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/7
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part1");

    let amplifier_controller_software = load_program(input)?;

    let mut largest_output = 0;

    let mut phase_settings = vec![0, 1, 2, 3, 4];
    let heap = permutohedron::Heap::new(&mut phase_settings);;

    for phase_settings in heap {
        let output = run_through_amplifiers(&amplifier_controller_software, &phase_settings)?;

        if output > largest_output {
            largest_output = output;
        }
    }

    Ok(largest_output)
}

/// See: https://adventofcode.com/2019/day/7#part2
#[wasm_bindgen]
pub fn part2(input: &str) -> Result<i32, JsValue> {
    Timer::new("rust::part2");

    let amplifier_controller_software = load_program(input)?;

    let mut largest_output = 0;

    let mut phase_settings = vec![5, 6, 7, 8, 9];
    let heap = permutohedron::Heap::new(&mut phase_settings);;

    for phase_settings in heap {
        let output = run_through_amplifiers_with_feedback(&amplifier_controller_software, &phase_settings)?;

        if output > largest_output {
            largest_output = output;
        }
    }

    Ok(largest_output)
}

fn run_through_amplifiers(amplifier_controller_software: &[i32], phase_settings: &[i32]) -> Result<i32, &'static str> {
    let mut runtime = vec![0; amplifier_controller_software.len()];

    let mut signal = 0;

    for phase_setting in phase_settings {
        runtime.copy_from_slice(amplifier_controller_software);
        let mut amp = Computer::new(&mut runtime);

        amp.put_input(*phase_setting);
        amp.put_input(signal);
        amp.run()?;

        signal = amp.get_output()
            .ok_or("amplifier controller software did not output any value")?;
    }

    Ok(signal)
}

fn run_through_amplifiers_with_feedback(amplifier_controller_software: &[i32], phase_settings: &[i32]) -> Result<i32, &'static str> {
    let mut program_amp_1 = vec![0; amplifier_controller_software.len()];
    program_amp_1.copy_from_slice(amplifier_controller_software);

    let mut amp_1 = Computer::new(&mut program_amp_1);
    amp_1.put_input(*phase_settings.get(0).unwrap());

    let mut program_amp_2 = vec![0; amplifier_controller_software.len()];
    program_amp_2.copy_from_slice(amplifier_controller_software);

    let mut amp_2 = Computer::new(&mut program_amp_2);
    amp_2.put_input(*phase_settings.get(1).unwrap());

    let mut program_amp_3 = vec![0; amplifier_controller_software.len()];
    program_amp_3.copy_from_slice(amplifier_controller_software);

    let mut amp_3 = Computer::new(&mut program_amp_3);
    amp_3.put_input(*phase_settings.get(2).unwrap());

    let mut program_amp_4 = vec![0; amplifier_controller_software.len()];
    program_amp_4.copy_from_slice(amplifier_controller_software);

    let mut amp_4 = Computer::new(&mut program_amp_4);
    amp_4.put_input(*phase_settings.get(3).unwrap());

    let mut program_amp_5 = vec![0; amplifier_controller_software.len()];
    program_amp_5.copy_from_slice(amplifier_controller_software);

    let mut amp_5 = Computer::new(&mut program_amp_5);
    amp_5.put_input(*phase_settings.get(4).unwrap());

    let mut signal = 0;

    loop {
        amp_1.put_input(signal);
        amp_1.run()?;

        signal = amp_1.get_output()
            .ok_or("amp 1 did not have any output")?;

        amp_2.put_input(signal);
        amp_2.run()?;

        signal = amp_2.get_output()
            .ok_or("amp 2 did not have any output")?;

        amp_3.put_input(signal);
        amp_3.run()?;

        signal = amp_3.get_output()
            .ok_or("amp 3 did not have any output")?;

        amp_4.put_input(signal);
        amp_4.run()?;

        signal = amp_4.get_output()
            .ok_or("amp 4 did not have any output")?;

        amp_5.put_input(signal);
        amp_5.run()?;

        signal = amp_5.get_output()
            .ok_or("amp 5 did not have any output")?;

        if amp_5.get_state() == State::HALTED {
            break;
        }
    }

    Ok(signal)
}
