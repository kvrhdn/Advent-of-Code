use aoc_runner_derive::aoc;

type Pos = crate::grid::Pos<i32>;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Eq, PartialEq)]
struct Ferry {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Ferry {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Cell::Floor,
                        'L' => Cell::Empty,
                        '#' => Cell::Occupied,
                        _ => panic!("Unexpected character {}", c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        }
    }

    fn get(&self, pos: Pos) -> Option<&Cell> {
        self.grid
            .get(pos.x as usize)
            .and_then(|row| row.get(pos.y as usize))
    }

    fn set(&mut self, pos: Pos, seat: Cell) {
        self.grid[pos.x as usize][pos.y as usize] = seat;
    }

    fn occupied_seats_in_line_of_sight(&self, start: Pos, max_range: usize) -> u32 {
        let directions = &[
            Pos::at(1, 1),
            Pos::at(1, 0),
            Pos::at(1, -1),
            Pos::at(0, 1),
            Pos::at(0, -1),
            Pos::at(-1, 1),
            Pos::at(-1, 0),
            Pos::at(-1, -1),
        ];

        let mut count = 0;

        for &dir in directions {
            let mut pos = start;
            for _ in 0..max_range {
                pos += dir;

                match self.get(pos) {
                    Some(Cell::Occupied) => {
                        count += 1;
                        break;
                    }
                    Some(Cell::Floor) => {
                        continue;
                    }
                    _ => {
                        break;
                    }
                }
            }
        }

        count
    }

    fn evolve<F>(&self, target: &mut Self, f: F)
    where
        F: Fn(Pos, Cell) -> Option<Cell>,
    {
        for row in 0..self.height {
            for col in 0..self.width {
                let pos = Pos::at(row as i32, col as i32);
                let curr = self.get(pos).unwrap();

                if let Some(new_value) = f(pos, *curr) {
                    target.set(pos, new_value);
                } else {
                    target.set(pos, *curr);
                }
            }
        }
    }

    fn evolve_with_rules_part1(&self, target: &mut Self) {
        self.evolve(target, |pos, curr| match curr {
            Cell::Empty if self.occupied_seats_in_line_of_sight(pos, 1) == 0 => {
                Some(Cell::Occupied)
            }
            Cell::Occupied if self.occupied_seats_in_line_of_sight(pos, 1) >= 4 => {
                Some(Cell::Empty)
            }
            _ => None,
        });
    }

    fn evolve_with_rules_part2(&self, target: &mut Self) {
        self.evolve(target, |pos, curr| match curr {
            Cell::Empty if self.occupied_seats_in_line_of_sight(pos, usize::MAX) == 0 => {
                Some(Cell::Occupied)
            }
            Cell::Occupied if self.occupied_seats_in_line_of_sight(pos, usize::MAX) >= 5 => {
                Some(Cell::Empty)
            }
            _ => None,
        });
    }

    fn seats_occupied(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&&s| s == Cell::Occupied).count())
            .sum()
    }
}

fn parse_input(input: &str) -> Ferry {
    Ferry::parse(input)
}

#[aoc(day11, part1)]
fn solve_part1(input: &str) -> usize {
    let mut buffer_a = parse_input(input);
    let mut buffer_b = buffer_a.clone();

    loop {
        buffer_a.evolve_with_rules_part1(&mut buffer_b);
        buffer_b.evolve_with_rules_part1(&mut buffer_a);

        if buffer_a == buffer_b {
            return buffer_a.seats_occupied();
        }
    }
}

#[aoc(day11, part2)]
fn solve_part2(input: &str) -> usize {
    let mut buffer_a = parse_input(input);
    let mut buffer_b = buffer_a.clone();

    loop {
        buffer_a.evolve_with_rules_part2(&mut buffer_b);
        buffer_b.evolve_with_rules_part2(&mut buffer_a);

        if buffer_a == buffer_b {
            return buffer_a.seats_occupied();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        assert_eq!(solve_part1(input), 37);
        assert_eq!(solve_part2(input), 26);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day11.txt");

        assert_eq!(solve_part1(input), 2261);
        assert_eq!(solve_part2(input), 2039);
    }
}
