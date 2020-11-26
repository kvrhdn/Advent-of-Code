use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn solve_part1(input: &str) -> i32 {
    parse_input(input).map(fuel_required).sum()
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> i32 {
    parse_input(input).map(total_fuel_required).sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.lines().map(|l| l.parse::<i32>().unwrap())
}

fn fuel_required(mass: i32) -> i32 {
    // integer dision already rounds down
    (mass / 3) - 2
}

fn total_fuel_required(mass: i32) -> i32 {
    let fuel = fuel_required(mass);
    if fuel <= 0 {
        return 0;
    }

    fuel + total_fuel_required(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_fuel_required() {
        #[rustfmt::skip]
        let examples = vec![
            (12, 2),
            (14, 2),
            (1969, 654),
            (100756, 33583),
        ];
        for (input, expected) in examples {
            assert_eq!(fuel_required(input), expected);
        }
    }

    #[test]
    fn examples_total_fuel_required() {
        #[rustfmt::skip]
        let examples = vec![
            (14, 2),
            (1969, 966),
            (100756, 50346),
        ];
        for (input, expected) in examples {
            assert_eq!(total_fuel_required(input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day1.txt");

        assert_eq!(solve_part1(input), 3266516);
        assert_eq!(solve_part2(input), 4896902);
    }
}
