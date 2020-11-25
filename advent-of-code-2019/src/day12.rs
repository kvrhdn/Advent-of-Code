use aoc_runner_derive::aoc;
use universe::*;

mod universe;

/// See: https://adventofcode.com/2019/day/12
#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> Result<i32, &'static str> {
    let mut universe = parse_input(input)?;

    universe.step(1000);

    Ok(universe.total_energy())
}

/// See: https://adventofcode.com/2019/day/12#part2
#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> Result<u64, &'static str> {
    let mut universe = parse_input(input)?;

    Ok(universe.find_cycle_time())
}

fn parse_input(input: &str) -> Result<Universe, &'static str> {
    let positions_result: Result<Vec<_>, _> = input.trim_end().lines().map(parse_line).collect();
    let positions = positions_result?;

    if positions.len() != 4 {
        return Err("this implementation expects exactly 4 moons");
    }

    Ok(Universe::new(&positions))
}

fn parse_line(line: &str) -> Result<(i32, i32, i32), &'static str> {
    // example input: <x=-3, y=-2, z=-4>

    let index_x = 2usize + line.find("x=").ok_or("could not parse input")?;
    let index_comma = line.find(", y=").ok_or("could not parse input")?;

    let x = line[index_x..index_comma]
        .parse::<i32>()
        .map_err(|_| "could not parse value of x")?;

    let index_y = 2usize + line.find("y=").ok_or("could not parse input")?;
    let index_comma = line.find(", z=").ok_or("could not parse input")?;

    let y = line[index_y..index_comma]
        .parse::<i32>()
        .map_err(|_| "could not parse value of y")?;

    let index_z = 2usize + line.find("z=").ok_or("could not parse input")?;
    let index_comma = line.find('>').ok_or("could not parse input")?;

    let z = line[index_z..index_comma]
        .parse::<i32>()
        .map_err(|_| "could not parse value of z")?;

    Ok((x, y, z))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let test_cases = vec![
            ("<x=-1, y=0, z=2>", (-1, 0, 2)),
            ("<x=2, y=-10, z=-7>", (2, -10, -7)),
            ("<x=4, y=-8, z=8>", (4, -8, 8)),
            ("<x=3, y=5, z=-1>", (3, 5, -1)),
        ];

        for (input, expected) in test_cases {
            let got = parse_line(input).unwrap();

            assert_eq!(got, expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day12.txt");

        assert_eq!(solve_part1(input), Ok(10055));
        assert_eq!(solve_part2(input), Ok(374307970285176));
    }
}
