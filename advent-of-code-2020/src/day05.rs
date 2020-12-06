use aoc_runner_derive::aoc;
use itertools::{
    Itertools,
    MinMaxResult::{MinMax, NoElements, OneElement},
};

// (row, col)
type Seat = (u16, u16);
type SeatId = u16;

fn parse_seat(input: &str) -> Seat {
    let (row, col) = input.split_at(7);

    (parse_binary(row, 'B'), parse_binary(col, 'R'))
}

fn parse_binary(input: &str, high: char) -> u16 {
    let mut value = 0;

    for c in input.chars() {
        value *= 2;

        if c == high {
            value += 1;
        }
    }

    value
}

fn seat_id(seat: Seat) -> SeatId {
    seat.0 * 8 + seat.1
}

fn process_input(input: &str) -> impl Iterator<Item = SeatId> + '_ {
    input.lines().map(parse_seat).map(seat_id)
}

#[aoc(day5, part1)]
fn solve_part1(input: &str) -> SeatId {
    process_input(input).max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> u32 {
    let seats = process_input(input).collect::<Vec<_>>();

    // we assume the seat IDs are consecutive from min to max (except for the one missing seat)
    let (min, max) = match seats.iter().minmax() {
        MinMax(min, max) => (*min as u32, *max as u32),
        OneElement(_) | NoElements => unreachable!(),
    };
    let expected_sum = ((max - min + 1) * (min + max)) / 2;

    let actual_sum = seats.iter().map(|&s| s as u32).sum::<u32>();

    expected_sum - actual_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        let examples = vec![
            ("BFFFBBFRRR", (70, 7), 567),
            ("FFFBBBFRRR", (14, 7), 119),
            ("BBFFBBFRLL", (102, 4), 820),
        ];

        for (input, expected_seat, expected_seat_id) in examples {
            let seat = parse_seat(input);

            assert_eq!(seat, expected_seat);
            assert_eq!(seat_id(seat), expected_seat_id);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day5.txt");

        assert_eq!(solve_part1(input), 828);
        assert_eq!(solve_part2(input), 565);
    }
}
