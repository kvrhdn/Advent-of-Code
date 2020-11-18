use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!(format!("Unexpected character {}", c)),
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut floor = 0i32;

    for (i, dir) in input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!(format!("Unexpected character {}", c)),
        })
        .enumerate()
    {
        floor += dir;

        if floor == -1 {
            return i as u32 + 1;
        }
    }
    panic!("Never reached floor -1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        let examples = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (input, expected) in examples {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn examples_part2() {
        #[rustfmt::skip]
        let examples = vec![
            (")", 1),
            ("()())", 5)
        ];

        for (input, expected) in examples {
            assert_eq!(solve_part2(input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day1.txt");

        assert_eq!(solve_part1(input), 280);
        assert_eq!(solve_part2(input), 1797);
    }
}
