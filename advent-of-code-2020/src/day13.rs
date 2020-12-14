use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut lines = input.lines();

    let earliest_timestamp = lines.next().unwrap().parse::<u32>().unwrap();

    lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|b| b.parse::<u32>().ok())
        .map(|bus| {
            let time_since_last_bus = earliest_timestamp % bus;
            let time_till_next = bus - time_since_last_bus;

            (bus, time_till_next)
        })
        .min_by_key(|&(_, time_till_next)| time_till_next)
        .map(|(bus, time_till_next)| bus * time_till_next)
        .unwrap()
}

#[aoc(day13, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, b)| match b.parse::<usize>() {
            Ok(bus) => Some((i, bus)),
            Err(_) => None,
        })
        .fold((0, 1), |(mut solution, product), (i, bus)| {
            while (solution + i) % bus != 0 {
                solution += product;
            }
            (solution, product * bus)
        })
        .0
}

// A bruteforce solution for my input (runs in about 11 minutes)
#[aoc(day13, part2, bruteforce)]
fn solve_part2_bruteforce(_: &str) -> usize {
    // step using the largest bus ID
    let steps = 787;
    let mut answer = 768;

    let print_step = 10_000_000_000_000;
    let mut print = print_step;

    loop {
        answer += steps;

        if answer > print {
            println!("Currently at: {}", answer);
            print += print_step;
        }

        if (answer + 50) % 571 == 0
            && (answer + 9) % 41 == 0
            && (answer + 13) % 37 == 0
            && (answer + 48) % 29 == 0
            && (answer + 42) % 23 == 0
            && answer % 19 == 0
            && (answer + 67) % 17 == 0
            && (answer + 32) % 13 == 0
        {
            return answer;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "\
939
7,13,x,x,59,x,31,19";

        assert_eq!(solve_part1(input), 295);
    }

    #[test]
    fn examples_part2() {
        let examples = vec![
            ("0\n7,13,x,x,59,x,31,19", 1068781),
            ("0\n17,x,13,19", 3417),
            ("0\n67,7,59,61", 754018),
            ("0\n67,x,7,59,61", 779210),
            ("0\n67,7,x,59,61", 1261476),
            ("0\n1789,37,47,1889", 1202161486),
        ];

        for (input, expected) in examples {
            assert_eq!(solve_part2(input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day13.txt");

        assert_eq!(solve_part1(input), 3997);
        assert_eq!(solve_part2(input), 500033211739354);
    }
}
