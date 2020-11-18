use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct Replacement {
    from: String,
    to: String,
}

impl Replacement {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<from>\w+) => (?P<to>\w+)$").unwrap();
        }
        let captures = RE.captures(s).unwrap();

        Self {
            from: captures.name("from").unwrap().as_str().to_string(),
            to: captures.name("to").unwrap().as_str().to_string(),
        }
    }
}

#[aoc_generator(day19, part1)]
fn parse_input(input: &str) -> (Vec<Replacement>, String) {
    let mut lines = input.lines();

    let replacements = lines
        .by_ref()
        .take_while(|&l| l != "")
        .map(|l| Replacement::parse(l))
        .collect();

    let sequence = lines.next().unwrap().to_string();

    (replacements, sequence)
}

// separate aoc_generator for part1 and part2 because it's not possible to
// borrow String between both parts...
#[aoc_generator(day19, part2)]
fn parse_input_p2(input: &str) -> (Vec<Replacement>, String) {
    parse_input(input)
}

#[aoc(day19, part1)]
fn solve_part1(input: &(Vec<Replacement>, String)) -> usize {
    let (replacements, sequence) = input;

    let mut molecules = HashSet::new();

    for replacement in replacements {
        let indices = sequence
            .match_indices(&replacement.from)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        for index in indices {
            let pre = sequence.split_at(index).0;
            let suf = sequence.split_at(index + replacement.from.len()).1;

            let molecule = [pre, &replacement.to, suf].join("");

            molecules.insert(molecule);
        }
    }

    molecules.len()
}

#[aoc(day19, part2)]
fn solve_part2(input: &(Vec<Replacement>, String)) -> u32 {
    let (replacements, sequence) = input;

    let mut processed: HashSet<String> = HashSet::new();
    let mut todo: HashMap<String, u32> = HashMap::new();

    todo.insert(sequence.clone(), 0);

    println!("Sequence len: {}", sequence.len());

    loop {
        // randomly grab some of the smallest molecules, we _assume_ that our
        // path to "e" will use the fastest decreasing molecules. This is not
        // guaranteed to work (i.e. if we follow a dead end). To avoid this we
        // could perhaps dynamically increase the size of the batch, creating a
        // more diverse mix when we don't progress anymore.
        let smallest_len = todo.keys().map(|k| k.len()).min().unwrap();
        let smallest_molecules = todo
            .iter()
            .filter(|(k, _)| k.len() == smallest_len)
            .take(50)
            // clone these molecules so we can insert into todo while looping
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<_>>();

        let smallest_molecules_len = smallest_molecules.len();

        for (m, gen) in smallest_molecules {
            for r in replacements {
                let indices = m.match_indices(&r.to).map(|(i, _)| i).collect::<Vec<_>>();

                for index in indices {
                    let pre = m.split_at(index).0;
                    let suf = m.split_at(index + r.to.len()).1;

                    let new = [pre, &r.from, suf].join("");

                    // we've already seen this molecule
                    if processed.contains(&new) || todo.contains_key(&new) {
                        continue;
                    }

                    todo.insert(new, gen + 1);
                }
            }

            // mark as processed
            processed.insert(m);
        }

        println!(
            "processing {:>2} of {:>5}, done: {:>5} - current min len: {}",
            smallest_molecules_len,
            todo.len(),
            processed.len(),
            smallest_len,
        );

        if let Some(&gen) = todo.get("e") {
            return gen;
        }

        todo.retain(|k, _| !processed.contains(k));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacement_parse() {
        let examples = vec![
            (
                "H => HO",
                Replacement {
                    from: "H".into(),
                    to: "HO".into(),
                },
            ),
            (
                "H => OH",
                Replacement {
                    from: "H".into(),
                    to: "OH".into(),
                },
            ),
            (
                "O => HH",
                Replacement {
                    from: "O".into(),
                    to: "HH".into(),
                },
            ),
        ];

        for (input, expected) in examples {
            assert_eq!(Replacement::parse(input), expected);
        }
    }

    #[test]
    fn example_part1() {
        let input = r#"H => HO
H => OH
O => HH

HOH"#;

        let parsed_input = parse_input(input);
        assert_eq!(solve_part1(&parsed_input), 4);
    }

    #[test]
    fn example_part2() {
        let input = r#"e => H
e => O
H => HO
H => OH
O => HH

HOHOHO"#;

        let parsed_input = parse_input(input);
        assert_eq!(solve_part2(&parsed_input), 6);
    }

    #[test]
    fn real_input_part1() {
        let input = include_str!("../input/2015/day19.txt");
        let parsed_input = parse_input(input);

        assert_eq!(solve_part1(&parsed_input), 576);
    }

    #[test]
    #[ignore = "expensive"]
    fn real_input_part2() {
        let input = include_str!("../input/2015/day19.txt");

        let parsed_input = parse_input(input);

        assert_eq!(solve_part2(&parsed_input), 207);
    }
}
