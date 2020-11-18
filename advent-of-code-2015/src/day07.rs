use aoc_runner_derive::{aoc, aoc_generator};
use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Parameter {
    Value(u16),
    Wire(String),
}

impl Parameter {
    fn resolve(&self, wire_map: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Parameter::Value(v) => Some(*v),
            Parameter::Wire(w) => wire_map.get(w).copied(),
        }
    }
}

impl From<u16> for Parameter {
    fn from(v: u16) -> Self {
        Self::Value(v)
    }
}

impl From<&str> for Parameter {
    fn from(s: &str) -> Self {
        if let Ok(v) = s.parse() {
            Self::Value(v)
        } else {
            Self::Wire(s.to_string())
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Gate {
    Direct(Parameter),
    And(Parameter, Parameter),
    Or(Parameter, Parameter),
    LShift(Parameter, u8),
    RShift(Parameter, u8),
    Not(Parameter),
}

impl Gate {
    fn parse(s: &str) -> Self {
        let words: Vec<&str> = s.split(' ').collect();

        assert!(!words.is_empty() && words.len() <= 3);

        if words.len() == 1 {
            let value = *words.get(0).unwrap();
            return Self::Direct(value.into());
        }

        if words.len() == 2 {
            assert_eq!(words.get(0), Some(&"NOT"));

            let value = *words.get(1).unwrap();
            return Self::Not(value.into());
        }

        let op1 = *words.get(0).unwrap();
        let op2 = *words.get(2).unwrap();

        match *words.get(1).unwrap() {
            "AND" => Self::And(op1.into(), op2.into()),
            "OR" => Self::Or(op1.into(), op2.into()),
            "LSHIFT" => Self::LShift(op1.into(), op2.parse().unwrap()),
            "RSHIFT" => Self::RShift(op1.into(), op2.parse().unwrap()),
            w => panic!("Unknown operand: {}", w),
        }
    }

    fn resolve(&self, wire_map: &HashMap<String, u16>) -> Option<u16> {
        Some(match self {
            Gate::Direct(p) => p.resolve(wire_map)?,
            Gate::And(p1, w2) => p1.resolve(wire_map)? & w2.resolve(wire_map)?,
            Gate::Or(p1, w2) => p1.resolve(wire_map)? | w2.resolve(wire_map)?,
            Gate::LShift(p, shift) => p.resolve(wire_map)? << shift,
            Gate::RShift(p, shift) => p.resolve(wire_map)? >> shift,
            Gate::Not(p) => !p.resolve(wire_map)?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instruction {
    input: Gate,
    output: String,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+) -> ([a-z]+)").unwrap();
        }
        let captures = RE.captures(s).unwrap().unwrap();

        assert_eq!(captures.len(), 3, "Input: {}", s);

        Instruction {
            input: Gate::parse(captures.get(1).unwrap().as_str()),
            output: captures.get(2).unwrap().as_str().into(),
        }
    }
}

struct WireState {
    map: HashMap<String, u16>,
}

impl WireState {
    fn from_instructions(instructions: &[Instruction]) -> Self {
        let mut map = HashMap::new();

        'outer: loop {
            for ir in instructions {
                if map.contains_key(&ir.output) {
                    continue;
                }

                if let Some(v) = ir.input.resolve(&map) {
                    map.insert(ir.output.to_string(), v);
                }

                if map.len() == instructions.len() {
                    break 'outer;
                }
            }
        }

        Self { map }
    }

    fn get(&self, wire: &str) -> Option<u16> {
        self.map.get(wire).copied()
    }
}

#[aoc_generator(day7)]
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| Instruction::parse(l)).collect()
}

#[aoc(day7, part1)]
fn solve_part1(instructions: &[Instruction]) -> u16 {
    WireState::from_instructions(instructions).get("a").unwrap()
}

#[aoc(day7, part2)]
fn solve_part2(instructions: &[Instruction]) -> u16 {
    let signal_wire_a = WireState::from_instructions(instructions).get("a").unwrap();

    let mut manipulated_instructions = instructions.to_vec();
    manipulated_instructions
        .iter_mut()
        .filter(|ir| ir.output == "b")
        .for_each(|ir| ir.input = Gate::Direct(Parameter::Value(signal_wire_a)));

    WireState::from_instructions(&manipulated_instructions)
        .get("a")
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_parse() {
        let examples = vec![
            (
                "123 -> x",
                Instruction {
                    input: Gate::Direct(123.into()),
                    output: "x".into(),
                },
            ),
            (
                "ab -> y",
                Instruction {
                    input: Gate::Direct("ab".into()),
                    output: "y".into(),
                },
            ),
            (
                "x AND y -> d",
                Instruction {
                    input: Gate::And("x".into(), "y".into()),
                    output: "d".into(),
                },
            ),
            (
                "x OR y -> e",
                Instruction {
                    input: Gate::Or("x".into(), "y".into()),
                    output: "e".into(),
                },
            ),
            (
                "x LSHIFT 2 -> f",
                Instruction {
                    input: Gate::LShift("x".into(), 2),
                    output: "f".into(),
                },
            ),
            (
                "y RSHIFT 2 -> g",
                Instruction {
                    input: Gate::RShift("y".into(), 2),
                    output: "g".into(),
                },
            ),
            (
                "NOT x -> h",
                Instruction {
                    input: Gate::Not("x".into()),
                    output: "h".into(),
                },
            ),
        ];

        for (input, expected) in examples {
            assert_eq!(Instruction::parse(input), expected);
        }
    }

    #[test]
    fn example_part1() {
        let input = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#;

        let instructions = parse_instructions(input);

        let wire_state = WireState::from_instructions(&instructions);

        assert_eq!(wire_state.get("d"), Some(72));
        assert_eq!(wire_state.get("e"), Some(507));
        assert_eq!(wire_state.get("f"), Some(492));
        assert_eq!(wire_state.get("g"), Some(114));
        assert_eq!(wire_state.get("h"), Some(65412));
        assert_eq!(wire_state.get("i"), Some(65079));
        assert_eq!(wire_state.get("x"), Some(123));
        assert_eq!(wire_state.get("y"), Some(456));
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day7.txt");

        let instructions = parse_instructions(input);

        assert_eq!(solve_part1(&instructions), 16076);
        assert_eq!(solve_part2(&instructions), 2797);
    }
}
