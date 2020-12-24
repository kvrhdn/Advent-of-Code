use aoc_runner_derive::aoc;
use noisy_float::prelude::*;
use std::collections::HashMap;

type Pos = (R32, R32);

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
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Black,
    White,
}

impl Tile {
    fn flip(&mut self) {
        *self = match self {
            Tile::Black => Tile::White,
            Tile::White => Tile::Black,
        }
    }
}

fn parse_input(input: &str) -> HashMap<Pos, Tile> {
    let mut tiles = HashMap::new();

    for mut line in input.lines() {
        let mut directions = Vec::new();

        while !line.is_empty() {
            let (dir, remaining) = Direction::parse(line);

            directions.push(dir);
            line = remaining;
        }

        let (x, y) = directions.iter().fold((0.0, 0.0), |(x, y), dir| match dir {
            Direction::East => (x + 1.0, y),
            Direction::SouthEast => (x + 0.5, y - 0.5),
            Direction::SouthWest => (x - 0.5, y - 0.5),
            Direction::West => (x - 1.0, y),
            Direction::NorthWest => (x - 0.5, y + 0.5),
            Direction::NorthEast => (x + 0.5, y + 0.5),
        });

        tiles.entry((r32(x), r32(y))).or_insert(Tile::White).flip();
    }

    tiles
}

#[aoc(day24, part1)]
fn solve_part1(input: &str) -> usize {
    let tiles = parse_input(input);

    tiles.values().filter(|&&tile| tile == Tile::Black).count()
}

#[aoc(day24, part2)]
fn solve_part2(input: &str) -> i32 {
    let mut tiles = parse_input(input);

    // TODO

    0
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
        assert_eq!(solve_part2(input), 0);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day24.txt");

        assert_eq!(solve_part1(input), 282);
        assert_eq!(solve_part2(input), 0);
    }
}
