#[derive(Clone)]
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
                1 => {
                    let op1 = self.get(index + 1) as usize;
                    let op2 = self.get(index + 2) as usize;
                    let result = self.get(index + 3) as usize;

                    self.set(result, self.get(op1) + self.get(op2));
                }
                2 => {
                    let op1 = self.get(index + 1) as usize;
                    let op2 = self.get(index + 2) as usize;
                    let result = self.get(index + 3) as usize;

                    self.set(result, self.get(op1) * self.get(op2));
                }
                99 => break,
                _ => return Err("got unexpected opcode in intcode program"),
            }
            index += 4;
        }
        Ok(())
    }
}
