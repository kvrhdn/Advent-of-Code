use aoc_runner_derive::aoc;

type SeatId = u16;

fn parse_seat_id(input: &str) -> SeatId {
    input
        .chars()
        .fold(0, |acc, c| (acc * 2) + ((c == 'B') != (c == 'R')) as u16)
}

fn process_input(input: &str) -> impl Iterator<Item = SeatId> + '_ {
    input.lines().map(parse_seat_id)
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
            ("BFFFBBFRRR", 567),
            ("FFFBBBFRRR", 119),
            ("BBFFBBFRLL", 820),
        ];

        for (input, expected) in examples {
            assert_eq!(parse_seat_id(input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day5.txt");

        assert_eq!(solve_part1(input), 828);
        assert_eq!(solve_part2(input), 565);
    }
}
