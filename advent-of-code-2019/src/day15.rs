use crate::day15::repair_droid::RepairDroid;
use crate::grid::{Dir, Pos};
use aoc_runner_derive::aoc;
use repair_droid::StatusOutput::{AtOxygenSystem, HitAWall, MovedForward};
use std::collections::HashMap;

mod repair_droid;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Wall,
    Visited(u32), // steps from start position
}

type Grid = HashMap<Pos, Tile>;

fn get_neighbors(grid: &Grid, pos: Pos) -> impl Iterator<Item = (Pos, &Tile)> + '_ {
    vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right]
        .into_iter()
        .map(move |dir| pos.step(dir))
        .filter_map(move |pos| grid.get(&pos).map(|tile| (pos, tile)))
}

fn find_oxygen_system(droid: &mut RepairDroid, grid: &mut Grid) -> Pos {
    'outer: loop {
        // find a valid direction - always try to go left first
        let mut directions = vec![
            droid.get_curr_dir().turn_left(),
            droid.get_curr_dir(),
            droid.get_curr_dir().turn_right(),
            droid.get_curr_dir().turn_left().turn_left(),
        ];

        // filter out directions we know will fail
        directions.retain(|&dir| {
            let pos = droid.get_pos().step(dir);
            !matches!(grid.get(&pos), Some(Tile::Wall))
        });

        // calculate the steps for a given position, only works if adjacent to
        // another Visited tile
        let steps_for_pos = |grid: &Grid, pos: Pos| -> u32 {
            get_neighbors(grid, pos)
                .filter_map(|(_, tile)| match tile {
                    Tile::Visited(steps) => Some(*steps),
                    Tile::Wall => None,
                })
                .min()
                .unwrap()
                + 1
        };

        for dir in directions {
            match droid.try_move(dir).unwrap() {
                HitAWall => {
                    grid.insert(droid.get_pos().step(dir), Tile::Wall);
                }
                MovedForward => {
                    let steps = steps_for_pos(&grid, droid.get_pos());
                    grid.insert(droid.get_pos(), Tile::Visited(steps));
                    continue 'outer;
                }
                AtOxygenSystem => {
                    let steps = steps_for_pos(&grid, droid.get_pos());
                    grid.insert(droid.get_pos(), Tile::Visited(steps));
                    return droid.get_pos();
                }
            }
        }

        unreachable!("The repair droid is stuck!");
    }
}

fn fill_with_oxygen(grid: &mut Grid, start: Pos) -> u32 {
    let mut curr_minute = 0;

    let mut tiles_with_oxygen = HashMap::new();
    tiles_with_oxygen.insert(start, curr_minute);

    loop {
        let most_recent_oxygenized_tiles = tiles_with_oxygen
            .iter()
            .filter(|&(_, minute)| *minute == curr_minute)
            .map(|(pos, _)| *pos)
            .collect::<Vec<_>>();

        if most_recent_oxygenized_tiles.is_empty() {
            return curr_minute - 1;
        }

        curr_minute += 1;

        for tile in most_recent_oxygenized_tiles {
            get_neighbors(&grid, tile)
                .filter(|&(_, tile)| *tile != Tile::Wall)
                .map(|(pos, _)| pos)
                .for_each(|pos| {
                    // only insert a new value if empty
                    tiles_with_oxygen.entry(pos).or_insert(curr_minute);
                });
        }
    }
}

#[aoc(day15, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut repair_droid = RepairDroid::new_from_input(input).unwrap();

    let mut grid = HashMap::new();
    grid.insert(repair_droid.get_pos(), Tile::Visited(0));

    let oxygen_station = find_oxygen_system(&mut repair_droid, &mut grid);

    if let Some(Tile::Visited(steps)) = grid.get(&oxygen_station) {
        return *steps;
    }

    unreachable!();
}

#[aoc(day15, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut repair_droid = RepairDroid::new_from_input(input).unwrap();

    let mut grid = HashMap::new();
    grid.insert(repair_droid.get_pos(), Tile::Visited(0));

    let oxygen_station = find_oxygen_system(&mut repair_droid, &mut grid);

    fill_with_oxygen(&mut grid, oxygen_station)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day15.txt");

        assert_eq!(solve_part1(input), 222);
        assert_eq!(solve_part2(input), 394);
    }
}
