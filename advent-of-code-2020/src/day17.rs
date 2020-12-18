use aoc_runner_derive::aoc;
use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cube {
    Active,
    Inactive,
}

impl Default for Cube {
    fn default() -> Self {
        Cube::Inactive
    }
}

#[derive(Clone)]
struct Space<T, const SIZE: usize, const DIM: usize> {
    data: Vec<T>,
}

impl<T, const S: usize, const D: usize> Default for Space<T, S, D>
where
    T: Clone + Default,
{
    fn default() -> Self {
        Space {
            data: vec![Default::default(); (S + 1).pow(D as u32)],
        }
    }
}

impl<T, const S: usize, const D: usize> Deref for Space<T, S, D> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, const S: usize, const D: usize> Index<[usize; D]> for Space<T, S, D> {
    type Output = T;

    fn index(&self, index: [usize; D]) -> &Self::Output {
        let i = self.buf_index(index);
        &self.data[i]
    }
}

impl<T, const S: usize, const D: usize> IndexMut<[usize; D]> for Space<T, S, D> {
    fn index_mut(&mut self, index: [usize; D]) -> &mut Self::Output {
        let i = self.buf_index(index);
        &mut self.data[i]
    }
}

impl<T, const S: usize, const D: usize> Space<T, S, D> {
    fn buf_index(&self, index: [usize; D]) -> usize {
        index
            .iter()
            .enumerate()
            .map(|(i, value)| value * (S + 1).pow((D - i - 1) as u32))
            .sum()
    }
}

impl<T, const S: usize> Space<T, S, 3>
where
    T: PartialEq,
{
    fn count_neighbors(&self, index: [usize; 3], state: T) -> usize {
        let mut count = 0;

        for x in index[0] - 1..=index[0] + 1 {
            for y in index[1] - 1..=index[1] + 1 {
                for z in index[2] - 1..=index[2] + 1 {
                    if self[[x, y, z]] == state {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

impl<T, const S: usize> Space<T, S, 4>
where
    T: PartialEq,
{
    fn count_neighbors(&self, index: [usize; 4], state: T) -> usize {
        let mut count = 0;

        for x in index[0] - 1..=index[0] + 1 {
            for y in index[1] - 1..=index[1] + 1 {
                for z in index[2] - 1..=index[2] + 1 {
                    for w in index[3] - 1..=index[3] + 1 {
                        if self[[x, y, z, w]] == state {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}

fn parse_input<const S: usize, const D: usize>(input: &str) -> Space<Cube, S, D> {
    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let input_height = lines.len();
    let input_width = lines[0].len();

    let mut space: Space<Cube, S, D> = Default::default();
    let middle = S / 2;

    for (y, line) in lines.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '#' {
                let mut pos = [middle; D];
                pos[0] = middle - (input_width / 2) + x;
                pos[1] = middle - (input_height / 2) + y;

                space[pos] = Cube::Active;
            }
        }
    }

    space
}

const SIZE: usize = 22;

#[aoc(day17, part1)]
fn solve_part1(input: &str) -> usize {
    let mut space_a = parse_input::<SIZE, 3>(input);
    let mut space_b = space_a.clone();

    for _ in 0..6 {
        std::mem::swap(&mut space_a, &mut space_b);

        for x in 1..SIZE - 1 {
            for y in 1..SIZE - 1 {
                for z in 1..SIZE - 1 {
                    let pos = [x, y, z];

                    // we can't call count_neighbors from update because it is only valid for D == 3 || D == 4
                    let active_neighbors = space_a.count_neighbors(pos, Cube::Active);
                    update(&space_a, &mut space_b, pos, active_neighbors);
                }
            }
        }
    }

    count_active(&space_b)
}

#[aoc(day17, part2)]
fn solve_part2(input: &str) -> usize {
    let mut space_a = parse_input::<SIZE, 4>(input);
    let mut space_b = space_a.clone();

    for _ in 0..6 {
        std::mem::swap(&mut space_a, &mut space_b);

        for x in 1..SIZE - 1 {
            for y in 1..SIZE - 1 {
                for z in 1..SIZE - 1 {
                    for w in 1..SIZE - 1 {
                        let pos = [x, y, z, w];

                        // we can't call count_neighbors from update because it is only valid for D == 3 || D == 4
                        let active_neighbors = space_a.count_neighbors(pos, Cube::Active);
                        update(&space_a, &mut space_b, pos, active_neighbors);
                    }
                }
            }
        }
    }

    count_active(&space_b)
}
fn update<const S: usize, const D: usize>(
    from: &Space<Cube, S, D>,
    to: &mut Space<Cube, S, D>,
    pos: [usize; D],
    active_neighbors: usize,
) {
    to[pos] = match from[pos] {
        Cube::Active => {
            // the current position is also counted
            let active_neighbors = active_neighbors - 1;

            if active_neighbors == 2 || active_neighbors == 3 {
                Cube::Active
            } else {
                Cube::Inactive
            }
        }
        Cube::Inactive => {
            if active_neighbors == 3 {
                Cube::Active
            } else {
                Cube::Inactive
            }
        }
    };
}

fn count_active(space: &[Cube]) -> usize {
    space.iter().filter(|&&c| c == Cube::Active).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
.#.
..#
###";

        assert_eq!(solve_part1(input), 112);
        assert_eq!(solve_part2(input), 848);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day17.txt");

        assert_eq!(solve_part1(input), 333);
        assert_eq!(solve_part2(input), 2676);
    }
}
