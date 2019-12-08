use State::*;

/// Intcode is a program, ran by computers in spaceships.
/// 
/// It was first introduced in day 02 and has been extended on in day 05 and
/// day 07.
/// 
/// Execution model 
/// 
/// An Intcode program is an array of integers. The Intcode program starts by
/// executing the instruction at position 0. After executing the instruction,
/// the instruction pointer will increase by the number of values in the
/// instruction (a step can range between 2 and 4 positions). Notable exception
/// are jump instructions, which manipulate the instruction pointer directly.
/// 
/// An Intcode program can read and output data through the input and output
/// buffers respectively.
/// 
/// Instructions
/// 
/// Instructions consist of an opcode followed by a list of parameters. The
/// amount of parameters is dependent on the instruction.
/// The opcode of an instruction both contains the instruction type and the
/// parameter mode.
/// 
/// Opcodes
/// 
/// 1   ADD (>day 02)
///     - parameter 1 & 2 [direct or by ref]: operands
///     - parameter 3 [by ref]: location to store the result
/// 
/// 2   MULTIPLY (>day 02)
///     - parameter 1 & 2 [direct or by ref]: operands
///     - parameter 3 [by ref]: location to store the result
/// 
/// 3   INPUT (>day 05)
///     - read one value from the input buffer
///     - parameter 1 [by ref]: location to store the input
/// 
/// 4   OUTPUT (>day 05)
///     - parameter [by ref]: read a value
///     - store the read value in the output buffer
///
/// 5   JUMP-IF-TRUE (>day 05)
///     - parameter 1 [direct or by ref]: condition: if value is not zero
///     - parameter 2 [by ref]: value to set the instruction pointer to
/// 
/// 6   JUMP-IF-FALSE (>day 05)
///     - parameter 1 [direct or by ref]: condition: if value is zero
///     - parameter 2 [by ref]: value to set the instruction pointer to
/// 
/// 7   LESS-THAN (>day 05)
///   - parameter 1 & 2 [direct or by ref]: condition: check whether value 1 is smaller than value 2
///   - parameter 3 [by ref]: location to store the result
/// 
/// 8   EQUALS (>day 05)
///   - parameter 1 & 2 [direct or by ref]: condition: check whether value 1 is smaller than value 2
///   - parameter 3 [ref]: location to store the result
/// 
/// 99  HALT (>day 02)
///   - stops the program execution
/// 
/// Encountering an unknown opcode means something went wrong and may throw an expection.
/// 
/// Parameters
/// 
/// Parameters can be access in two manners:
/// 
///   - Direct use the value stored in memory directly
///   - By ref use the value stored in memory as address to load another value
///
pub struct Computer<'a> {
    ir_ptr: usize,
    state: State,
    memory: &'a mut [i32],
    input: Vec<i32>,
    output: Vec<i32>,
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

impl<'a> Computer<'a> {
    /// Initialize a new intcode computer with the given program.
    pub fn new(program: &'a mut [i32]) -> Self {
        Computer {
            ir_ptr: 0,
            state: READY,
            memory: program,
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    /// The state the program is in.
    pub fn get_state(&self) -> State {
        self.state
    }

    /// Add a value to the input buffer.
    pub fn put_input(&mut self, value: i32) {
        self.input.push(value);

        if self.state == BLOCKED {
            self.state = READY;
        }
    }

    /// Read one value from the output buffer, returns None if buffer is empty.
    pub fn get_output(&mut self) -> Option<i32> {
        if self.output.is_empty() {
            None
        } else {
            Some(self.output.remove(0))
        }
    }

    /// Returns a reference to the entire output buffer.
    pub fn get_output_buffer(&self) -> &[i32] {
        &self.output
    }

    /// Return the value stored in memory at position `addr`.
    pub fn get(&self, addr: usize) -> i32 {
        self.memory[addr]
    }

    /// Set the memory at position `addr` to `value`.
    pub fn set(&mut self, addr: usize, value: i32) {
        self.memory[addr] = value;
    }

    /// Return the value stored in memory at the position designated by the
    /// value at `addr`.
    fn get_by_ref(&self, addr: usize) -> i32 {
        self.memory[self.get(addr) as usize]
    }

    fn get_with_mode(&self, addr: usize, immediate: bool) -> i32 {
        if immediate {
            self.get(addr)
        } else {
            self.get_by_ref(addr)
        }
    }

    /// Set the memory at position designated by the value at `addr` to
    /// `value`.
    fn set_by_ref(&mut self, addr: usize, value: i32) {
        self.set(self.get(addr) as usize, value)
    }

    /// Run the program loaded in memory until program is halted, blocked or
    /// dead.
    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            let opcode = self.get(self.ir_ptr);

            match opcode % 100 {
                // add
                1 => {
                    let op1 = self.get_with_mode(self.ir_ptr + 1, is_digit_set(opcode, 2));
                    let op2 = self.get_with_mode(self.ir_ptr + 2, is_digit_set(opcode, 3));

                    let result = op1 + op2;

                    self.set_by_ref(self.ir_ptr + 3, result);
                    self.ir_ptr += 4;
                },
                // multiply
                2 => {
                    let op1 = self.get_with_mode(self.ir_ptr + 1, is_digit_set(opcode, 2));
                    let op2 = self.get_with_mode(self.ir_ptr + 2, is_digit_set(opcode, 3));

                    let result = op1 * op2;

                    self.set_by_ref(self.ir_ptr + 3, result);
                    self.ir_ptr += 4;
                },
                // input
                3 => {
                    if self.input.is_empty() {
                        self.state = BLOCKED;
                        return Ok(());
                    }

                    let value = self.input.remove(0);

                    self.set_by_ref(self.ir_ptr + 1, value);
                    self.ir_ptr += 2;
                },
                // output
                4 => {
                    self.output.push(self.get_with_mode(self.ir_ptr + 1, is_digit_set(opcode, 2)));
                    self.ir_ptr += 2;
                },
                // jump-if-true
                5 => {
                    if self.get_with_mode(self.ir_ptr + 1, is_digit_set(opcode, 2)) != 0 {
                        self.ir_ptr = self.get_with_mode(self.ir_ptr + 2, is_digit_set(opcode, 3)) as usize;
                    } else { 
                        self.ir_ptr += 3;
                    }
                },
                // jump-if-false
                6 => {
                    if self.get_with_mode(self.ir_ptr + 1, is_digit_set(opcode, 2)) == 0 {
                        self.ir_ptr = self.get_with_mode(self.ir_ptr + 2, is_digit_set(opcode, 3)) as usize;
                    } else { 
                        self.ir_ptr += 3;
                    }
                },
                // less-than
                7 => {
                    if self.get_with_mode(self.ir_ptr + 1, is_digit_set(opcode, 2)) < self.get_with_mode(self.ir_ptr + 2, is_digit_set(opcode, 3)) {
                        self.set_by_ref(self.ir_ptr + 3, 1);
                    } else {
                        self.set_by_ref(self.ir_ptr + 3, 0);
                    }
                    self.ir_ptr += 4;
                },
                // equals
                8 => {
                    if self.get_with_mode(self.ir_ptr + 1, is_digit_set(opcode, 2)) == self.get_with_mode(self.ir_ptr + 2, is_digit_set(opcode, 3)) {
                        self.set_by_ref(self.ir_ptr + 3, 1);
                    } else {
                        self.set_by_ref(self.ir_ptr + 3, 0);
                    }
                    self.ir_ptr += 4;
                },
                // halt
                99 => {
                    self.state = HALTED;
                    return Ok(());
                },
                _ => panic!("got unexpected opcode \"{}\" in intcode program at position \"{}\"", opcode, self.ir_ptr),
            }
        }
    }
}

/// Load a program from text.
pub fn load_program(input: &str) -> Result<Vec<i32>, &'static str> {
    input
        .trim_end()
        .split(',')
        .map(|l| {
            l.parse::<i32>()
                .map_err(|_| "could not parse input as integers")
        })
        .collect()
}

/// A digit is set if it is not null. Digits are counted from right-to-left,
/// starting with index 0.
fn is_digit_set(value: i32, digit: u32) -> bool {
    (value / 10i32.pow(digit)) % 10 > 0
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

        for (mut program, expected) in cases {
            let mut computer = Computer::new(&mut program);

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
            let mut program_copy = program.clone();
            let mut computer = Computer::new(&mut program_copy);

            computer.put_input(input);
            computer.run()
                .unwrap();

            assert_eq!(computer.get_output().unwrap(), expected);
        }
    }

    #[test]
    fn test_is_digit_set() {
        let cases = vec![
            (1023, 0, true),
            (1023, 1, true),
            (1023, 2, false),
            (1023, 3, true),
            (1023, 4, false),
        ];

        for (input, digit, expected) in cases {
            let got = is_digit_set(input, digit);

            if expected != got {
                panic!("is_digit_set({}, {}) = {}, expected {}", input, digit, got, expected)
            }
        }
    }
}
