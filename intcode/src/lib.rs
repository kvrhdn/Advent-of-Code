pub struct Computer<'a> {
    memory: &'a mut [u32],
}

impl <'a> Computer<'a> {
    /// Initialize a new intcode computer with the given program.
    pub fn new(program: &'a mut [u32]) -> Self {
        Computer { memory: program }
    }

    /// Return the value stored in memory at position `addr`.
    pub fn get(&self, addr: usize) -> u32 {
        self.memory[addr]
    }

    /// Set the memory at position `addr` to `value`.
    pub fn set(&mut self, addr: usize, value: u32) {
        self.memory[addr] = value;
    }

    /// Run the program loaded into memory until the halt instruction is reached
    /// or the program panics.
    pub fn run(&mut self) -> Result<(), &'static str> {
        let mut ir_ptr = 0;

        loop {
            match self.get(ir_ptr) {
                1 => self.apply_operation(ir_ptr + 1, ir_ptr + 2, ir_ptr + 3, |a, b| a + b),
                2 => self.apply_operation(ir_ptr + 1, ir_ptr + 2, ir_ptr + 3, |a, b| a * b),
                99 => return Ok(()),
                _ => return Err("got unexpected opcode in intcode program"),
            }
            ir_ptr += 4;
        }
    }

    fn apply_operation<F>(
        &mut self,
        index_op1: usize,
        index_op2: usize,
        index_result: usize,
        operation: F,
    ) where
        F: FnOnce(u32, u32) -> u32,
    {
        let op1 = self.get(index_op1) as usize;
        let op2 = self.get(index_op2) as usize;

        let result = operation(self.get(op1), self.get(op2));

        self.set(self.get(index_result) as usize, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_test_run() {
        let cases: Vec<(Vec<u32>, Vec<u32>)> = vec![
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
            (vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
            (vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]),
        ];

        for (mut input, expected) in cases {
            let mut computer = Computer::new(&mut input);

            computer.run().unwrap();

            assert_eq!(expected, computer.memory);
        }
    }
}
