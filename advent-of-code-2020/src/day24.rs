use aoc_runner_derive::aoc;
use std::{collections::HashSet, ops::Add};

/// A position in the a hex grid, following the doubled coordinates approach.
/// For a position to be valid, `col + row % 2 == 0` should be true.
/// Source: https://www.redblobgames.com/grids/hexagons/#coordinates-doubled
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Pos {
    col: i32,
    row: i32,
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn parse(s: &str) -> (Direction, &str) {
        if s.starts_with("ne") {
            return (Direction::NorthEast, &s[2..]);
        }
        if s.starts_with("nw") {
            return (Direction::NorthWest, &s[2..]);
        }
        if s.starts_with("se") {
            return (Direction::SouthEast, &s[2..]);
        }
        if s.starts_with("sw") {
            return (Direction::SouthWest, &s[2..]);
        }
        if s.starts_with("e") {
            return (Direction::East, &s[1..]);
        }
        if s.starts_with("w") {
            return (Direction::West, &s[1..]);
        }
        panic!("unrecognized direction: {}", s);
    }

    /// Returns a relative position corresponding to this direction.
    fn as_relative_pos(&self) -> Pos {
        match self {
            Direction::East => Pos { col: 2, row: 0 },
            Direction::West => Pos { col: -2, row: 0 },
            Direction::NorthEast => Pos { col: 1, row: -1 },
            Direction::SouthEast => Pos { col: 1, row: 1 },
            Direction::NorthWest => Pos { col: -1, row: -1 },
            Direction::SouthWest => Pos { col: -1, row: 1 },
        }
    }
}

fn parse_input(input: &str) -> HashSet<Pos> {
    let mut black_tiles = HashSet::new();

    for mut line in input.lines() {
        let mut pos: Pos = Pos { col: 0, row: 0 };

        while !line.is_empty() {
            let (dir, remaining) = Direction::parse(line);

            pos = pos + dir.as_relative_pos();
            line = remaining;
        }

        if black_tiles.contains(&pos) {
            black_tiles.remove(&pos);
        } else {
            black_tiles.insert(pos);
        }
    }

    black_tiles
}

#[aoc(day24, part1)]
fn solve_part1(input: &str) -> usize {
    let black_tiles = parse_input(input);

    black_tiles.len()
}

#[aoc(day24, part2)]
fn solve_part2(input: &str) -> usize {
    let mut black_tiles = parse_input(input);

    let count_neighbors = |tiles: &HashSet<Pos>, pos: &Pos| -> usize {
        vec![
            Direction::East,
            Direction::West,
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ]
        .into_iter()
        .map(|dir| *pos + dir.as_relative_pos())
        .filter(|pos| tiles.contains(pos))
        .count()
    };

    for _ in 0..100 {
        let mut new_tiles = HashSet::new();

        let (min_col, max_col, min_row, max_row) =
            black_tiles
                .iter()
                .fold((0, 0, 0, 0), |(min_col, max_col, min_row, max_row), pos| {
                    (
                        min_col.min(pos.col),
                        max_col.max(pos.col),
                        min_row.min(pos.row),
                        max_row.max(pos.row),
                    )
                });

        for row in min_row - 1..=max_row + 1 {
            let mut start_col = min_col;
            if (row + start_col) % 2 != 0 {
                start_col += 1;
            }

            for col in (start_col - 2..=max_col + 2).step_by(2) {
                let pos = Pos { col, row };

                let n = count_neighbors(&black_tiles, &pos);
                let is_black = black_tiles.contains(&pos);

                if is_black && (n == 1 || n == 2) {
                    new_tiles.insert(pos);
                }
                if !is_black && n == 2 {
                    new_tiles.insert(pos);
                }
            }
        }

        black_tiles = new_tiles;
    }

    black_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        assert_eq!(solve_part1(input), 10);
        assert_eq!(solve_part2(input), 2208);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day24.txt");

        assert_eq!(solve_part1(input), 282);
        assert_eq!(solve_part2(input), 3445);
    }
}
