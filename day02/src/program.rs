#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntcodeProgram {
    list: Vec<u32>,
}

impl From<Vec<u32>> for IntcodeProgram {
    fn from(list: Vec<u32>) -> IntcodeProgram {
        IntcodeProgram { list }
    }
}

impl IntcodeProgram {
    fn index_wrap(&self, i: usize) -> usize {
        i % self.list.len()
    }

    pub fn get(&self, i: usize) -> u32 {
        self.list[self.index_wrap(i)]
    }

    fn set(&mut self, i: usize, value: u32) {
        let index = self.index_wrap(i);
        self.list[index] = value;
    }

    pub fn set_noun_and_verb(&mut self, noun: u32, verb: u32) {
        self.set(1, noun);
        self.set(2, verb);
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        let mut index = 0;
        loop {
            match self.get(index) {
                1 => self.apply_operation(index + 1, index + 2, index + 3, |a, b| a + b),
                2 => self.apply_operation(index + 1, index + 2, index + 3, |a, b| a * b),
                99 => break,
                _ => return Err("got unexpected opcode in intcode program"),
            }
            index += 4;
        }
        Ok(())
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
        let cases = vec![
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
            (vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
            (vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]),
        ];

        for (input, expected) in cases {
            let mut program: IntcodeProgram = input.into();

            program.run().unwrap();

            assert_eq!(program, expected.into());
        }
    }
}
