use crate::grid::Pos;
use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashSet, iter};

struct TreeMap {
    trees: HashSet<Pos>,
    width: i32,
    height: i32,
}

impl TreeMap {
    fn parse_input(input: &str) -> Self {
        let trees = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(move |(x, _)| Pos::at(x as i32, y as i32))
            })
            .collect::<HashSet<_>>();

        let width = trees.iter().map(|pos| pos.x).max().unwrap() + 1;
        let height = trees.iter().map(|pos| pos.y).max().unwrap() + 1;

        Self {
            trees,
            width,
            height,
        }
    }

    fn is_tree(&self, pos: &Pos) -> bool {
        self.trees.contains(&Pos::at(pos.x % self.width, pos.y))
    }
}

#[rustfmt::skip]
fn diagonal_iter(slope: Pos) -> impl Iterator<Item = Pos> {
    iter::repeat(slope)
        .scan(Pos::at(0, 0), |curr_pos, slope| {
            *curr_pos += slope;
            Some(*curr_pos)
        })
}

fn trees_encountered(trees: &TreeMap, slope: Pos) -> usize {
    diagonal_iter(slope)
        .take_while(|&pos| pos.y < trees.height)
        .filter(|pos| trees.is_tree(pos))
        .count()
}

#[aoc_generator(day3, part1)]
fn parse_input_part1(input: &str) -> TreeMap {
    TreeMap::parse_input(input)
}

#[aoc_generator(day3, part2)]
fn parse_input_part2(input: &str) -> TreeMap {
    TreeMap::parse_input(input)
}

#[aoc(day3, part1)]
fn solve_part1(trees: &TreeMap) -> usize {
    trees_encountered(&trees, Pos::at(3, 1))
}

#[aoc(day3, part2)]
fn solve_part2(trees: &TreeMap) -> usize {
    let slopes = vec![
        Pos::at(1, 1),
        Pos::at(3, 1),
        Pos::at(5, 1),
        Pos::at(7, 1),
        Pos::at(1, 2),
    ];

    slopes
        .into_iter()
        .map(|slope| trees_encountered(&trees, slope))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

        let trees = TreeMap::parse_input(input);

        assert_eq!(solve_part1(&trees), 7);
        assert_eq!(solve_part2(&trees), 336);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day3.txt");

        let trees = TreeMap::parse_input(input);

        assert_eq!(solve_part1(&trees), 272);
        assert_eq!(solve_part2(&trees), 3898725600);
    }
}
