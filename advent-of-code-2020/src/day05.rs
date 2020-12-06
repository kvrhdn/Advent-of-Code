use aoc_runner_derive::aoc;

// (row, col)
type Seat = (u16, u16);
type SeatId = u16;

fn parse_seat(input: &str) -> Seat {
    let (row, col) = input.split_at(7);

    (parse_binary(row, 'B'), parse_binary(col, 'R'))
}

fn parse_binary(input: &str, high: char) -> u16 {
    input
        .chars()
        .fold(0, |acc, c| (acc * 2) + (c == high) as u16)
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
    // we assume the seat IDs are consecutive from min to max, except for the one missing seat

    #[rustfmt::skip]
    let (actual_sum, min, max) = process_input(input)
        .map(|seat_id| seat_id as u32)
        .fold((0, u32::MAX, u32::MIN), |(sum, min, max), seat_id| {
            (
                sum + seat_id,
                min.min(seat_id),
                max.max(seat_id),
            )
        },
    );

    let expected_sum = ((max - min + 1) * (min + max)) / 2;

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
