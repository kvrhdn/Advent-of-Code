use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<u32> {
    let mut joltages: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    // add wall joltage
    joltages.push(0);

    joltages.sort_unstable();

    // add device joltage
    joltages.push(joltages[joltages.len() - 1] + 3);

    joltages
}

#[aoc(day10, part1)]
fn solve_part1(joltages: &[u32]) -> u32 {
    // the difference between two joltages is guaranteed to be 1, 2 or 3
    let mut diffs = [0; 4];

    joltages
        .windows(2)
        .for_each(|w| diffs[(w[1] - w[0]) as usize] += 1);

    diffs[1] * diffs[3]
}

#[aoc(day10, part2)]
fn solve_part2(joltages: &[u32]) -> u64 {
    // keep track of the amount of branches up to the corresponding joltage
    let mut branches = vec![0u64; joltages.len()];
    branches[0] = 1;

    for i in 1..joltages.len() {
        let adapter = joltages[i];

        // check if previous joltages are in range and count sum of branches
        let mut count = 0;

        if i >= 3 && adapter - joltages[i - 3] <= 3 {
            count += branches[i - 3];
        }
        if i >= 2 && adapter - joltages[i - 2] <= 3 {
            count += branches[i - 2];
        }
        if i >= 1 && adapter - joltages[i - 1] <= 3 {
            count += branches[i - 1];
        }

        branches[i] = count;
    }

    branches[joltages.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let examples = vec![
            (
                "\
16
10
15
5
1
11
7
19
6
12
4",
                35,
                8,
            ),
            (
                "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3",
                220,
                19208,
            ),
        ];

        for (input, expected_part1, expected_part2) in examples {
            let input = parse_input(input);

            assert_eq!(solve_part1(&input), expected_part1);
            assert_eq!(solve_part2(&input), expected_part2);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day10.txt");
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 2240);
        assert_eq!(solve_part2(&input), 99214346656768);
    }
}
