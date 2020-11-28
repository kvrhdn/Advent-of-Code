use crate::grid::{distance, Pos};
use aoc_runner_derive::aoc;
use itertools::Itertools;
use noisy_float::prelude::*;
use std::collections::{HashMap, HashSet};

#[aoc(day10, part1)]
fn solve_part1(input: &str) -> usize {
    let map = parse_input(input);

    let (_, buckets) = find_best_location(&map);
    buckets.len()
}

#[aoc(day10, part2)]
fn solve_part2(input: &str) -> i32 {
    let map = parse_input(input);

    let (monitoring_station, mut buckets) = find_best_location(&map);

    // sort the buckets by distance internally
    buckets
        .values_mut()
        .for_each(|bucket| bucket.sort_unstable_by_key(|&pos| distance(monitoring_station, pos)));

    let mut angles_iter = buckets.keys().copied().sorted().cycle().peekable();

    // forward the iterator to 0
    while *angles_iter.peek().unwrap() != r32(0f32) {
        angles_iter.next();
    }

    let mut vaporization_count = 0;

    loop {
        let angle = angles_iter.next().unwrap();

        let bucket = buckets.get_mut(&angle).unwrap();
        if bucket.is_empty() {
            continue;
        }

        let removed = bucket.remove(0);
        vaporization_count += 1;

        if vaporization_count == 200 {
            return removed.x * 100 + removed.y;
        }
    }
}

type AsteroidMap = HashSet<Pos>;

fn parse_input(input: &str) -> AsteroidMap {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| *char == '#')
                .map(move |(x, _)| Pos::at(x as i32, y as i32))
        })
        .collect()
}

type AngleBuckets<'a> = HashMap<R32, Vec<&'a Pos>>;

fn bucket_into_angles<'a>(around: &Pos, map: &'a AsteroidMap) -> AngleBuckets<'a> {
    map.iter()
        .filter(|&pos| pos != around)
        .group_by(|&pos| angle_between(around, pos))
        .into_iter()
        .map(|(key, group)| (key, group.collect::<Vec<_>>()))
        .collect()
}

fn angle_between(from: &Pos, to: &Pos) -> R32 {
    let diff_x = (to.x - from.x) as f32;
    let diff_y = (to.y - from.y) as f32;

    // invert y so the upwards direction corresponds to 0
    r32(diff_x.atan2(-diff_y))
}

fn find_best_location(map: &AsteroidMap) -> (&Pos, AngleBuckets) {
    map.iter()
        .map(|pos| (pos, bucket_into_angles(pos, &map)))
        .max_by_key(|(_, buckets)| buckets.len())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        let examples = vec![
            (
                r#".#..#
.....
#####
....#
...##"#,
                Pos::at(3, 4),
                8,
            ),
            (
                r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#,
                Pos::at(5, 8),
                33,
            ),
            (
                r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#,
                Pos::at(1, 2),
                35,
            ),
            (
                r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#,
                Pos::at(6, 3),
                41,
            ),
            (
                r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#,
                Pos::at(11, 13),
                210,
            ),
        ];

        for (input, expected_pos, expected_count) in examples {
            let map = parse_input(input);

            let (position, buckets) = find_best_location(&map);
            assert_eq!(position, &expected_pos);
            assert_eq!(buckets.len(), expected_count);
        }
    }

    #[test]
    fn example_part2() {
        let input = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#;

        assert_eq!(solve_part2(input), 802);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day10.txt");

        assert_eq!(solve_part1(input), 286);
        assert_eq!(solve_part2(input), 504);
    }
}
