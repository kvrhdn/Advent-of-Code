use aoc_runner_derive::{aoc, aoc_generator};
use std::{
    collections::HashMap,
    mem,
    ops::{Index, IndexMut},
};
use GridTransformation::{FlipHorizontal, FlipVertical, RotateClockwise};

enum GridTransformation {
    FlipHorizontal,
    FlipVertical,
    RotateClockwise,
}

/// Grid is a square grid of size by size.
#[derive(Clone, Debug)]
struct Grid<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Copy + Default,
{
    fn new(size: usize) -> Self {
        Self {
            size,
            data: vec![T::default(); size * size],
        }
    }

    fn top_border(&self) -> Vec<T> {
        (0..self.size).map(move |i| self[(i, 0)]).collect()
    }

    fn bottom_border(&self) -> Vec<T> {
        (0..self.size)
            .map(move |i| self[(i, self.size - 1)])
            .collect()
    }

    fn left_border(&self) -> Vec<T> {
        (0..self.size).map(move |i| self[(0, i)]).collect()
    }

    fn right_border(&self) -> Vec<T> {
        (0..self.size)
            .map(move |i| self[(self.size - 1, i)])
            .collect()
    }

    fn all_borders(&self) -> Vec<Vec<T>> {
        vec![
            self.top_border(),
            self.bottom_border(),
            self.left_border(),
            self.right_border(),
        ]
    }

    fn apply_transformation(&mut self, transformation: GridTransformation) {
        let mut new = Self::new(self.size);

        for x in 0..self.size {
            for y in 0..self.size {
                let source_index = match transformation {
                    FlipHorizontal => (x, self.size - 1 - y),
                    FlipVertical => (self.size - 1 - x, y),
                    RotateClockwise => (self.size - 1 - y, x),
                };
                new[(x, y)] = self[source_index];
            }
        }

        let _ = mem::replace(&mut self.data, new.data);
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[(index.0 * self.size) + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[(index.0 * self.size) + index.1]
    }
}

type TileId = u32;

#[derive(Clone, Debug)]
struct Tile {
    id: TileId,
    data: Grid<bool>,
}

impl Tile {
    fn parse_from_input(s: &str) -> Self {
        let mut lines = s.lines();

        let mut tile = Self {
            id: 0,
            data: Grid::new(10),
        };

        let tile_id = lines.next().unwrap();
        tile.id = serde_scan::scan!("Tile {}:" <- tile_id).unwrap();

        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    tile.data[(i, j)] = true;
                }
            }
        }

        tile
    }
}

/// ConnectedTile is a tile that is connected to other tiles of an image.
#[derive(Debug)]
struct ConnectedTile {
    id: TileId,
    data: Grid<bool>,
    top: Option<TileId>,
    bottom: Option<TileId>,
    left: Option<TileId>,
    right: Option<TileId>,
}

impl ConnectedTile {
    fn find_connections(tiles: &[Tile]) -> impl Iterator<Item = ConnectedTile> + '_ {
        tiles.iter().map(move |tile| {
            let find_connecting_tile = |border: Vec<bool>| -> Option<u32> {
                let reversed_border = {
                    let mut reversed_border = border.clone();
                    reversed_border.reverse();
                    reversed_border
                };

                for other_tile in tiles.iter() {
                    if tile.id == other_tile.id {
                        continue;
                    }

                    for other_border in other_tile.data.all_borders() {
                        if border == other_border || reversed_border == other_border {
                            return Some(other_tile.id);
                        }
                    }
                }
                None
            };

            Self {
                id: tile.id,
                data: tile.data.clone(),
                top: find_connecting_tile(tile.data.top_border()),
                bottom: find_connecting_tile(tile.data.bottom_border()),
                left: find_connecting_tile(tile.data.left_border()),
                right: find_connecting_tile(tile.data.right_border()),
            }
        })
    }

    fn connection_count(&self) -> usize {
        [self.top, self.bottom, self.left, self.right]
            .iter()
            .filter(|c| c.is_some())
            .count()
    }

    fn apply_transformation(&mut self, transformation: GridTransformation) {
        match transformation {
            RotateClockwise => {
                let prev_top = self.top;
                self.top = self.right;
                self.right = self.bottom;
                self.bottom = self.left;
                self.left = prev_top;
            }
            FlipHorizontal => {
                mem::swap(&mut self.top, &mut self.bottom);
            }
            FlipVertical => {
                mem::swap(&mut self.left, &mut self.right);
            }
        }
        self.data.apply_transformation(transformation);
    }
}

/// Image is square 2D grid stiched together from a collection of tiles.
#[derive(Debug)]
struct Image {
    data: Grid<bool>,
}

impl Image {
    fn assemble_from_tiles(tiles: &[Tile]) -> Image {
        let size = (tiles.len() as f32).sqrt();
        if size.fract() != 0.0 {
            panic!("the amount of tiles must be a power of 2");
        }
        let size = size as usize;

        let mut tiles = ConnectedTile::find_connections(tiles)
            .map(|c| (c.id, c))
            .collect::<HashMap<_, _>>();

        let mut ordered_tiles: Vec<ConnectedTile> = Vec::new();

        for row in 0..size {
            let tile_id = if row == 0 {
                // first row: find our first corner piece
                tiles
                    .values()
                    // find a corner tile of which only BOTTOM and RIGHT are connected (naive but works)
                    .find(|c| {
                        matches!(
                            (c.top, c.bottom, c.left, c.right),
                            (None, Some(_), None, Some(_))
                        )
                    })
                    .map(|c| c.id)
                    .expect("could not find the top-right corner")
            } else {
                // we already have a row, lookup bottom from row above
                ordered_tiles[(row - 1) * size].bottom.unwrap()
            };

            let mut start_tile = tiles.remove(&tile_id).unwrap();

            // rotate our piece so top matches with the row above
            if row != 0 {
                let prev_row_start = ordered_tiles[(row - 1) * size].id;

                while start_tile.top != Some(prev_row_start) {
                    start_tile.apply_transformation(RotateClockwise);
                }
            }
            // right must have a connection
            if start_tile.right.is_none() {
                start_tile.apply_transformation(FlipVertical);
            }

            debug_assert!(start_tile.left.is_none());

            let mut prev = start_tile;

            // attach tiles to prev until the row is full
            for col in 1..size {
                let mut next = tiles.remove(&prev.right.unwrap()).unwrap();

                while next.left != Some(prev.id) {
                    next.apply_transformation(RotateClockwise);
                }

                if prev.data.right_border() != next.data.left_border() {
                    next.apply_transformation(FlipHorizontal);
                }

                debug_assert!(prev.data.right_border() == next.data.left_border());

                if row != 0 {
                    let tile_above = &ordered_tiles[(row - 1) * size + col];

                    debug_assert!(next.top.unwrap() == tile_above.id);
                    debug_assert!(next.data.top_border() == tile_above.data.bottom_border());
                }

                ordered_tiles.push(prev);
                prev = next;
            }

            debug_assert!(prev.right.is_none());

            // wrap up this row
            ordered_tiles.push(prev);
        }

        let mut image = Self {
            // tile have size 10, minus bordesr -> 8
            data: Grid::new(size * 8),
        };

        // write image data
        for y in 0..size {
            for x in 0..size {
                let offset = (x * 8, y * 8);

                let tile_data = &ordered_tiles[y * size + x].data;

                // copy without the borders
                for y2 in 0..tile_data.size - 2 {
                    for x2 in 0..tile_data.size - 2 {
                        image.data[(offset.0 + x2, offset.1 + y2)] = tile_data[(x2 + 1, y2 + 1)];
                    }
                }
            }
        }

        // // print entire image
        // for y in 0..image.data.size {
        //     for x in 0..image.data.size {
        //         print!("{}", if image.data[(x, y)] { '#' } else { '.' });
        //     }
        //     println!();
        // }

        image
    }

    fn find_pattern(&self, pattern: &str) -> usize {
        let pattern: Vec<(usize, usize)> = pattern
            .lines()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(move |(x, _)| (x, y))
            })
            .collect::<Vec<_>>();

        let pattern_width = *pattern.iter().map(|(x, _)| x).max().unwrap();
        let pattern_height = *pattern.iter().map(|(_, y)| y).max().unwrap();

        let mut count = 0;

        // scan entire image for the pattern
        for y in 0..self.data.size - pattern_height {
            for x in 0..self.data.size - pattern_width {
                let mut found = true;

                for p in &pattern {
                    if !self.data[(x + p.0, y + p.1)] {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;
                }
            }
        }

        count
    }
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Vec<Tile> {
    input.split("\n\n").map(Tile::parse_from_input).collect()
}

#[aoc(day20, part1)]
fn solve_part1(tiles: &[Tile]) -> u64 {
    ConnectedTile::find_connections(tiles)
        // tiles with two connections will be corners
        .filter(|c| c.connection_count() == 2)
        .map(|c| c.id as u64)
        .product()
}

#[aoc(day20, part2)]
fn solve_part2(tiles: &[Tile]) -> usize {
    let sea_monster = r#"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "#;

    let mut image = Image::assemble_from_tiles(tiles);

    for i in 0..8 {
        let found = image.find_pattern(sea_monster);
        if found > 0 {
            let total = image.data.data.iter().filter(|&&v| v).count();
            let sea_monster = sea_monster.chars().filter(|&c| c == '#').count();

            return total - (found * sea_monster);
        }

        image.data.apply_transformation(RotateClockwise);
        if i == 3 {
            image.data.apply_transformation(FlipVertical);
        }
    }

    panic!("could not find any sea monsters!");
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
        assert_eq!(solve_part2(&input), 273);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day20.txt").trim_end();
        let input = parse_input(input);

        assert_eq!(solve_part1(&input), 13983397496713);
        assert_eq!(solve_part2(&input), 2424);
    }
}
