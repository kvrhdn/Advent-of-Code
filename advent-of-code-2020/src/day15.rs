use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<u32> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn memory_game(start_sequence: &[u32], max_turns: usize) -> u32 {
    // turn 0 does not exist, so 0 is considered none
    let mut prev_numbers = vec![0; max_turns];

    let mut last_spoken = start_sequence[0];

    for (turn, &nr) in start_sequence.iter().enumerate().skip(1) {
        prev_numbers[last_spoken as usize] = turn;
        last_spoken = nr;
    }

    for turn in start_sequence.len()..max_turns {
        let speak = match prev_numbers[last_spoken as usize] {
            0 => 0,
            last_turn => (turn - last_turn) as u32,
        };

        prev_numbers[last_spoken as usize] = turn;
        last_spoken = speak;
    }

    last_spoken
}

#[aoc(day15, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    memory_game(input, 2020)
}

#[aoc(day15, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    memory_game(input, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "0,3,6";
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 436);
    }

    #[test]
    fn examples_part2() {
        let examples = vec![
            ("0,3,6", 175594),
            ("1,3,2", 2578),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ];

        for (input, expected) in examples {
            let input = parse_input(input);

            assert_eq!(solve_part2(&input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day15.txt").trim_end();
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 260);
        assert_eq!(solve_part2(&input), 950);
    }
}
