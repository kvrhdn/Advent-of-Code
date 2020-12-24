use aoc_runner_derive::aoc;
use noisy_float::prelude::*;
use std::collections::HashSet;

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

fn parse_input(input: &str) -> HashSet<Pos> {
    let mut tiles = HashSet::new();

    for mut line in input.lines() {
        let mut pos: Pos = (r32(0.0), r32(0.0));

        while !line.is_empty() {
            let (dir, remaining) = Direction::parse(line);

            pos = match dir {
                Direction::East => (pos.0 + 1.0, pos.1),
                Direction::SouthEast => (pos.0 + 0.5, pos.1 - 0.5),
                Direction::SouthWest => (pos.0 - 0.5, pos.1 - 0.5),
                Direction::West => (pos.0 - 1.0, pos.1),
                Direction::NorthWest => (pos.0 - 0.5, pos.1 + 0.5),
                Direction::NorthEast => (pos.0 + 0.5, pos.1 + 0.5),
            };

            line = remaining;
        }

        if tiles.contains(&pos) {
            tiles.remove(&pos);
        } else {
            tiles.insert(pos);
        }
    }

    tiles
}

#[aoc(day24, part1)]
fn solve_part1(input: &str) -> usize {
    let tiles = parse_input(input);

    tiles.len()
}

#[aoc(day24, part2)]
fn solve_part2(input: &str) -> usize {
    let mut tiles = parse_input(input);

    let count_neighbors = |tiles: &HashSet<Pos>, pos: Pos| -> u32 {
        tiles.contains(&(pos.0 + 1.0, pos.1)) as u32
            + tiles.contains(&(pos.0 + 0.5, pos.1 - 0.5)) as u32
            + tiles.contains(&(pos.0 - 0.5, pos.1 - 0.5)) as u32
            + tiles.contains(&(pos.0 - 1.0, pos.1)) as u32
            + tiles.contains(&(pos.0 - 0.5, pos.1 + 0.5)) as u32
            + tiles.contains(&(pos.0 + 0.5, pos.1 + 0.5)) as u32
    };

    for _ in 0..100 {
        let mut new_tiles = HashSet::new();

        let (min_x, max_x, min_y, max_y): (R32, R32, R32, R32) = tiles.iter().fold(
            (r32(0.0), r32(0.0), r32(0.0), r32(0.0)),
            |(min_x, max_x, min_y, max_y), &(x, y)| {
                (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
            },
        );

        let mut y = min_y - 1.0f32;
        while y < max_y + 1.0 {
            let mut x = min_x - 1.0f32;

            while x < max_x + 1.0 {
                let n = count_neighbors(&tiles, (x, y));
                let is_black = tiles.contains(&(x, y));

                if is_black && (n == 1 || n == 2) {
                    new_tiles.insert((x, y));
                }
                if !is_black && n == 2 {
                    new_tiles.insert((x, y));
                }

                x += 0.5;
            }

            y += 0.5;
        }

        tiles = new_tiles;
    }

    tiles.len()
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
