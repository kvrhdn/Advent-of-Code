use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// See: https://adventofcode.com/2019/day/2
#[wasm_bindgen]
pub fn part1(input: &str) -> Result<u32, JsValue> {
    let mut list: IntcodeProgram = parse_input(input)?.into();

    list.set(1, 12);
    list.set(2, 2);

    list.run()?;

    Ok(list.get(0))
}

/// See: https://adventofcode.com/2019/day/2#part2
#[wasm_bindgen]
pub fn part2(_input: &str) -> Result<i32, JsValue> {
    Ok(0)
}

/// Parse the input as a list of integers.
fn parse_input(input: &str) -> Result<Vec<u32>, &'static str> {
    input
        .trim_end()
        .split(',')
        .map(|l| {
            println!("{}", l);
            l.parse::<u32>()
                .map_err(|_| "could not parse input as integers")
        })
        .collect()
}

struct IntcodeProgram {
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

    fn get(&self, i: usize) -> u32 {
        self.list[self.index_wrap(i)]
    }

    fn set(&mut self, i: usize, value: u32) {
        let index = self.index_wrap(i);
        self.list[index] = value;
    }

    fn run(&mut self) -> Result<(), &'static str> {
        let mut index = 0;
        loop {
            match self.get(index) {
                1 => {
                    let op1 = self.get(index + 1) as usize;
                    let op2 = self.get(index + 2) as usize;
                    let result = self.get(index + 3) as usize;

                    self.set(result, self.get(op1) + self.get(op2));
                },
                2 => {
                    let op1 = self.get(index + 1) as usize;
                    let op2 = self.get(index + 2) as usize;
                    let result = self.get(index + 3) as usize;

                    self.set(result, self.get(op1) * self.get(op2));
                },
                99 => break,
                _ => return Err("got unexpected opcode in intcode program"),
            }

            index += 4;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_input() {
        assert_eq!(crate::parse_input("123,25,0\n"), Ok(vec![123, 25, 0]));
        assert!(crate::parse_input("123,-5,0\n").is_err());
        assert!(crate::parse_input("123,a,0\n").is_err());
    }
}
