use aoc_runner_derive::aoc;

fn mine_advent_coin(secret_key: &str, verify: fn(&[u8; 16]) -> bool) -> u32 {
    let mut appendix = 0;

    let mut md5_input = secret_key.trim_end().to_string();
    let replacement_index = md5_input.len();

    loop {
        md5_input.replace_range(replacement_index.., &appendix.to_string());
        let digest = md5::compute(&md5_input);

        if verify(&digest) {
            return appendix;
        }

        appendix += 1;
    }
}

#[aoc(day4, part1)]
fn solve_part1(input: &str) -> u32 {
    mine_advent_coin(input, |d| {
        d.starts_with(&[0, 0]) && (d.get(2).unwrap() & 0xF0) == 0
    })
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> u32 {
    mine_advent_coin(input, |d| d.starts_with(&[0, 0, 0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        #[rustfmt::skip]
        let examples = vec![
            ("abcdef", 609043),
            ("pqrstuv", 1048970),
        ];

        for (input, expected) in examples {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day4.txt");

        assert_eq!(solve_part1(input), 117946);
        assert_eq!(solve_part2(input), 3938038);
    }
}
