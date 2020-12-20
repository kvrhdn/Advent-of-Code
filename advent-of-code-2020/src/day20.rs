use std::ops::{Index, IndexMut};

use aoc_runner_derive::{aoc, aoc_generator};

/// Tile is a 10x10 boolean grid that can be indexed by `(usize, usize)`.
#[derive(Debug)]
struct Tile {
    id: u32,
    data: [bool; 10 * 10],
}

impl Tile {
    fn parse(s: &str) -> Self {
        let mut tile = Self {
            id: 0,
            data: [false; 10 * 10],
        };

        let mut lines = s.lines();

        let tile_id = lines.next().unwrap();
        tile.id = serde_scan::scan!("Tile {}:" <- tile_id).unwrap();

        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    tile[(i, j)] = true;
                }
            }
        }

        tile
    }

    fn borders<'a>(&'a self) -> Vec<Vec<bool>> {
        vec![
            (0..10usize).map(|i| self[(i, 0usize)]).collect(),
            (0..10usize).map(|i| self[(i, 9usize)]).collect(),
            (0..10usize).map(|i| self[(0usize, i)]).collect(),
            (0..10usize).map(|i| self[(9usize, i)]).collect(),
        ]
    }
}

impl Index<(usize, usize)> for Tile {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[(index.0 * 10) + index.1]
    }
}

impl IndexMut<(usize, usize)> for Tile {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[(index.0 * 10) + index.1]
    }
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Vec<Tile> {
    input.split("\n\n").map(Tile::parse).collect()
}

#[aoc(day20, part1)]
fn solve_part1(tiles: &[Tile]) -> u64 {
    let mut corners: Vec<u32> = Vec::new();

    for tile in tiles {
        let mut connections = 0;

        'border: for border in tile.borders() {
            let reversed_border = {
                let mut reversed_border = border.clone();
                reversed_border.reverse();
                reversed_border
            };

            for other_tile in tiles {
                if tile.id == other_tile.id {
                    continue;
                }

                for other_border in other_tile.borders() {
                    if border == other_border || reversed_border == other_border {
                        connections += 1;
                        continue 'border;
                    }
                }
            }
        }

        if connections == 2 {
            corners.push(tile.id);
        }
    }

    corners.iter().map(|&v| v as u64).product()
}

#[aoc(day20, part2)]
fn solve_part2(_tiles: &[Tile]) -> u64 {
    // TODO
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 20899048083289);
        assert_eq!(solve_part2(&input), 0);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day20.txt").trim_end();
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 13983397496713);
        assert_eq!(solve_part2(&input), 0);
    }
}
