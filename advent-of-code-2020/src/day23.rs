use aoc_runner_derive::{aoc, aoc_generator};
use std::str;

/// Play the crab game 🦀.
fn play_game(start_sequence: &[u8], total_cups: usize, moves: u32) -> Vec<usize> {
    // every index points to the next cup, i.e. cups[5] returns the neighbor of cup 5
    let mut cups = vec![0usize; total_cups + 1];

    let mut prev = start_sequence[0] as usize;

    // build up the cups data structure
    for &value in start_sequence.iter().skip(1) {
        cups[prev] = value as usize;
        prev = value as usize;
    }
    for value in start_sequence.len() + 1..total_cups + 1 {
        cups[prev] = value;
        prev = value;
    }

    cups[prev] = start_sequence[0] as usize;

    let mut curr = start_sequence[0] as usize;
    let mut pick_up = [0; 3];

    for _ in 0..moves {
        // populate pick_up
        let mut c = cups[curr];
        for p in pick_up.iter_mut() {
            *p = c;
            c = cups[c];
        }

        // point curr to the first cup after pick_up
        cups[curr] = c;

        let mut dest = curr - 1;
        while dest < 1 || pick_up.contains(&dest) {
            if dest == 0 {
                dest = total_cups;
            } else {
                dest -= 1;
            }
        }

        // point end of pick_up to the cup after dest
        cups[pick_up[2]] = cups[dest];
        // point dest to the start of pick_up
        cups[dest] = pick_up[0];

        curr = cups[curr];
    }

    cups
}

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<u8> {
    input.as_bytes().iter().map(|b| b - b'0').collect()
}

#[aoc(day23, part1)]
fn solve_part1(input: &[u8]) -> String {
    let cups = play_game(input, 9, 100);

    let mut result = String::with_capacity(8);

    let mut c = cups[1];
    for _ in 0..8 {
        result.push((b'0' + c as u8) as char);
        c = cups[c];
    }

    result
}

#[aoc(day23, part2)]
fn solve_part2(input: &[u8]) -> usize {
    let cups = play_game(input, 1_000_000, 10_000_000);

    let first = cups[1];
    let second = cups[first];

    first * second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "389125467";
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), "67384529");
        assert_eq!(solve_part2(&input), 149245887792);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day23.txt").trim_end();
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), "46978532");
        assert_eq!(solve_part2(&input), 163035127721);
    }
}
