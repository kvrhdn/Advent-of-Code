use State::*;

/// Intcode is a program, ran by computers in spaceships.
/// 
/// It was first introduced in day 02 and has been extended on in day 05, day
/// 07 and finally day 09.
/// 
/// Execution model 
/// 
/// An Intcode program is an array of integers. The Intcode program starts by
/// executing the instruction at position 0. After executing the instruction,
/// the instruction pointer will increase by the number of values in the
/// instruction (an instruction can contain various amount of parameters).
/// Notable exception are jump instructions, which manipulate the instruction
/// pointer directly.
/// 
/// An Intcode program can read and write data through the input and output
/// buffers respectively.
/// 
/// Instructions
/// 
/// Instructions consist of an opcode followed by a list of parameters. The
/// amount of parameters is dependent on the instruction.
/// The opcode of an instruction both contains the instruction type and the
/// parameter mode. Parameters can be accessed in two manners:
/// 
///   0 Position mode (aka by reference): use the value stored in memory as
///     address to load another value.
///   1 Immediate mode: use the value stored in memory directly.
///   2 Relative mode: use the value stored in memory as address but relative
///     to a global relative base.
/// 
/// Opcodes
/// 
/// 1   ADD (>day 02)
///     - parameter 1 & 2: operands
///     - parameter 3: location to store the result
/// 
/// 2   MULTIPLY (>day 02)
///     - parameter 1 & 2: operands
///     - parameter 3: location to store the result
/// 
/// 3   INPUT (>day 05)
///     - read one value from the input buffer
///     - parameter 1: location to store the input
/// 
/// 4   OUTPUT (>day 05)
///     - parameter: read a value
///     - store the read value in the output buffer
///
/// 5   JUMP-IF-TRUE (>day 05)
///     - parameter 1: condition: if value is not zero
///     - parameter 2: value to set the instruction pointer to
/// 
/// 6   JUMP-IF-FALSE (>day 05)
///     - parameter 1: condition: if value is zero
///     - parameter 2: value to set the instruction pointer to
/// 
/// 7   LESS-THAN (>day 05)
///   - parameter 1 & 2: condition: check whether value 1 is smaller than value 2
///   - parameter 3: location to store the result
/// 
/// 8   EQUALS (>day 05)
///   - parameter 1 & 2: condition: check whether value 1 is smaller than value 2
///   - parameter 3 [ref]: location to store the result
/// 
/// 9   ADJUST-RELATIVE-BASE (>day 09)
///   - parameter 1: the value to adjust the relative base with (increase or decrease)
/// 
/// 99  HALT (>day 02)
///   - stops the program execution
/// 
/// Encountering an unknown opcode means something went wrong and may throw an expection.
/// 
pub struct Computer {
    ir_ptr: usize,
    state: State,
    memory: Vec<i64>,
    memory_relative_base: i64,
    input: Vec<i64>,
    output: Vec<i64>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum State {
    /// The program is ready to run
    READY,
    /// Waiting for input to continue
    BLOCKED,
    /// The program has reached a HALT instruction (99) and has finished
    HALTED,
    /// The program has crashed and can't continue execution
    DEAD,
}

impl Computer {
    /// Initialize a new intcode computer with the given program.
    pub fn new(program: Vec<i64>) -> Self {
        Computer {
            ir_ptr: 0,
            state: READY,
            memory: program,
            memory_relative_base: 0,
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    /// Initialize a new intcode computer with the given input.
    pub fn new_from_input(input: &str) -> Result<Self, &'static str> {
        Ok(Self::new(load_program(input)?))
    }

    /// The state the program is in.
    pub fn get_state(&self) -> State {
        self.state
    }

    /// Add a value to the input buffer.
    pub fn put_input(&mut self, value: i64) {
        self.input.push(value);

        if self.state == BLOCKED {
            self.state = READY;
        }
    }

    /// Read and consume one value from the output buffer, returns None if the
    /// output buffer is empty.
    pub fn get_output(&mut self) -> Option<i64> {
        if self.output.is_empty() {
            None
        } else {
            Some(self.output.remove(0))
        }
    }

    /// Returns a read-only reference to the entire output buffer. To consume
    /// the buffer use `get_output()`.
    pub fn get_output_buffer(&self) -> &[i64] {
        &self.output
    }

    /// Return the value stored in memory at position `addr`.
    pub fn get(&self, addr: usize) -> i64 {
        if addr >= self.memory.len() {
            0
        } else {
            self.memory[addr]
        }
    }

    /// Set the memory at position `addr` to `value`.
    pub fn set(&mut self, addr: usize, value: i64) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        self.memory[addr] = value;
    }

    /// Get the parameter at ir_ptr + index, using the parameter mode as marked
    /// by the opcode at ir_ptr. Index starts at 1 for the first parameter of
    /// an instruction.
    fn get_param(&self, index: u32) -> Result<i64, &'static str> {
        let mode = get_digit(self.get(self.ir_ptr), index + 1);
        let param_value = self.get(self.ir_ptr + index as usize);

        let value = match mode {
            // position mode
            0 => self.get(param_value as usize),
            // immediate mode
            1 => param_value,
            // relative mode
            2 => self.get((self.memory_relative_base + param_value) as usize),
            // unknown
            _ => return Err("Unsupported address mode"),
        };
        Ok(value)
    }

    /// Set the memory at position designated by the value at ir_ptr + index
    /// to value. Index starts at 1 for the first parameter of an instruction.
    /// Does not support immediate or position mode and always uses position
    /// mode.
    fn set_by_param(&mut self, index: u32, value: i64) -> Result<(), &'static str> {
        let mode = get_digit(self.get(self.ir_ptr), index + 1);
        let param_value = self.get(self.ir_ptr + index as usize);

        match mode {
            // position mode
            0 => self.set(param_value as usize, value),
            // immediate mode
            1 => return Err("Immediate mode is not supported when storing values"),
            // relative mode
            2 => self.set((self.memory_relative_base + param_value) as usize, value),
            // unknown
            _ => return Err("Unsupported address mode"),
        };
        Ok(())
    }

    /// Run the program loaded in memory until program is halted, blocked or
    /// dead.
    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            match self.get(self.ir_ptr) % 100 {
                // ADD
                1 => {
                    let value = self.get_param(1)? + self.get_param(2)?;
                    self.set_by_param(3, value)?;

                    self.ir_ptr += 4;
                },
                // MULTIPLY
                2 => {
                    let value = self.get_param(1)? * self.get_param(2)?;
                    self.set_by_param(3, value)?;

                    self.ir_ptr += 4;
                },
                // INPUT
                3 => {
                    if self.input.is_empty() {
                        self.state = BLOCKED;
                        return Ok(());
                    }
                    let value = self.input.remove(0);
                    self.set_by_param(1, value)?;

                    self.ir_ptr += 2;
                },
                // OUTPUT
                4 => {
                    self.output.push(self.get_param(1)?);

                    self.ir_ptr += 2;
                },
                // JUMP-IF-TRUE
                5 => {
                    if self.get_param(1)? != 0 {
                        self.ir_ptr = self.get_param(2)? as usize;
                    } else {
                        self.ir_ptr += 3;
                    }
                },
                // JUMP-IF-FALSE
                6 => {
                    if self.get_param(1)? == 0 {
                        self.ir_ptr = self.get_param(2)? as usize;
                    } else { 
                        self.ir_ptr += 3;
                    }
                },
                // LESS-THAN
                7 => {
                    // a bool when cast as integer is guaranteed to be 1 or 0: https://doc.rust-lang.org/std/primitive.bool.html
                    let value = (self.get_param(1)? < self.get_param(2)?) as i64;
                    self.set_by_param(3, value)?;

                    self.ir_ptr += 4;
                },
                // EQUALS
                8 => {
                    let value = (self.get_param(1)? == self.get_param(2)?) as i64;
                    self.set_by_param(3, value)?;

                    self.ir_ptr += 4;
                },
                // ADJUST-RELATIVE-BASE
                9 => {
                    self.memory_relative_base += self.get_param(1)?;

                    self.ir_ptr += 2;
                },
                // halt
                99 => {
                    self.state = HALTED;
                    return Ok(());
                },
                _ => {
                    self.state = DEAD;
                    return Err("reached an unsupported opcode, can't proceed")
                },
            }
        }
    }

    /// Run the program loaded in memory in diagnostics mode, in this mode a
    /// test input is provided to the program. The diagnostics program is
    /// expected test various parts of the intcode computer and output values
    /// indicating how far off the result of the test are from the expected
    /// values.
    /// Finally, the program will output a diagnostic code. This code is
    /// returned from run_diagnostics_test if all other tests were successful.
    /// The computer has to be reset before running another program.
    pub fn run_diagnostics_test(&mut self, test_input: i64) -> Result<i64, &'static str> {
        self.put_input(test_input);
        self.run()?;

        let output = self.get_output_buffer();
        if output.is_empty() {
            return Err("diagnostics test created no output");
        }
    
        if output[0..output.len() - 1]
                .iter()
                .any(|&val| val != 0) {
            return Err("diagnostics test failed");
        }

        Ok(*output.last().unwrap())
    }
}

/// Load a program from text.
pub fn load_program(input: &str) -> Result<Vec<i64>, &'static str> {
    input
        .trim_end()
        .split(',')
        .map(|l| {
            l.parse::<i64>()
                .map_err(|_| "could not parse input as integers")
        })
        .collect()
}

/// A digit is set if it is not null. Digits are counted from right-to-left,
/// starting with index 0.
fn get_digit(value: i64, digit: u32) -> i64 {
    (value / 10i64.pow(digit)) % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_program() {
        assert_eq!(load_program("123,25,0\n"), Ok(vec![123, 25, 0]));
        assert_eq!(load_program("123,-5,0\n"), Ok(vec![123, -5, 0]));
        assert!(load_program("123,a,0\n").is_err());
    }


    #[test]
    fn examples_day_02() {
        let cases = vec![
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
            (vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
            (vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]),
        ];

        for (program, expected) in cases {
            let mut computer = Computer::new(program);

            computer.run()
                .unwrap();

            assert_eq!(expected, computer.memory);
        }
    }

    #[test]
    fn examples_day_05() {
        let is_equal_to_8_position_mode = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let is_less_than_8_position_mode = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let is_equal_to_8_immediate_mode = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let is_less_than_8_immediate_mode = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let is_not_zero_position_mode = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let is_not_zero_immediate_mode = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        // if < 0: 999; if =0: 1000; if >0: 1001
        let compare_with_zero = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];

        let test_cases = vec![
            (&is_equal_to_8_position_mode, 8, 1),
            (&is_equal_to_8_position_mode, 1, 0),
            (&is_less_than_8_position_mode, 5, 1),
            (&is_less_than_8_position_mode, 8, 0),
            (&is_equal_to_8_immediate_mode, 8, 1),
            (&is_equal_to_8_immediate_mode, 1, 0),
            (&is_less_than_8_immediate_mode, 5, 1),
            (&is_less_than_8_immediate_mode, 8, 0),
            (&is_not_zero_position_mode, 0, 0),
            (&is_not_zero_position_mode, 5, 1),
            (&is_not_zero_immediate_mode, 0, 0),
            (&is_not_zero_immediate_mode, 5, 1),
            (&compare_with_zero, -7, 999),
            (&compare_with_zero, 8, 1000),
            (&compare_with_zero, 9, 1001),
        ];

        for (program, input, expected) in test_cases {
            let program_copy = program.clone();
            let mut computer = Computer::new(program_copy);

            computer.put_input(input);
            computer.run()
                .unwrap();

            assert_eq!(computer.get_output().unwrap(), expected);
        }
    }

    #[test]
    fn examples_day_09() {
        let produces_a_copy_of_itself = vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
        let produces_a_copy_of_itself_result = produces_a_copy_of_itself.clone();

        let outputs_a_16_digit_number: Vec<i64> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let outputs_a_16_digit_number_result: Vec<i64> = vec![1219070632396864];

        let outputs_large_number: Vec<i64> = vec![104, 1125899906842624, 99];
        let outputs_large_number_result: Vec<i64> = vec![1125899906842624];

        let test_cases = vec![
            (&produces_a_copy_of_itself, &produces_a_copy_of_itself_result),
            (&outputs_a_16_digit_number, &outputs_a_16_digit_number_result),
            (&outputs_large_number, &outputs_large_number_result),
        ];

        for (program, expected) in test_cases {
            let program_copy = program.clone();
            let mut computer = Computer::new(program_copy);

            computer.run()
                .unwrap();

            assert_eq!(expected, &computer.get_output_buffer());
        }
    }

    #[test]
    fn test_get_digit() {
        let cases = vec![
            (1023, 0, 3),
            (1023, 1, 2),
            (1023, 2, 0),
            (1023, 3, 1),
            (1023, 4, 0),
        ];

        for (input, digit, expected) in cases {
            let got = get_digit(input, digit);

            if expected != got {
                panic!("is_digit_set({}, {}) = {}, expected {}", input, digit, got, expected)
            }
        }
    }
}
