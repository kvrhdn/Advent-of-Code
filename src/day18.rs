use aoc_runner_derive::aoc;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Light {
    On,
    Off,
}

impl Light {
    fn parse(c: &char) -> Self {
        match c {
            '#' => Light::On,
            '.' => Light::Off,
            _ => panic!("Unexpected character: {}", c),
        }
    }

    fn evolved(&self, neighbors: usize) -> Self {
        match self {
            Light::On => {
                if neighbors == 2 || neighbors == 3 {
                    Light::On
                } else {
                    Light::Off
                }
            }
            Light::Off => {
                if neighbors == 3 {
                    Light::On
                } else {
                    Light::Off
                }
            }
        }
    }
}

impl fmt::Display for Light {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Light::On => write!(fmt, "#"),
            Light::Off => write!(fmt, "."),
        }
    }
}

struct Grid {
    width: usize,
    data: Vec<Light>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();

        let data = input
            .lines()
            .flat_map(|line| line.chars().map(|c| Light::parse(&c)))
            .collect();

        Self { width, data }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    fn surrounding_neighbors(&self, x: usize, y: usize) -> usize {
        let mut neighbors = 0;

        for y2 in y.saturating_sub(1)..=y + 1 {
            for x2 in x.saturating_sub(1)..=x + 1 {
                // don't go out-of-bounds
                if x2 >= self.width || y2 >= self.width {
                    continue;
                }
                // don't count the current light
                if x2 == x && y2 == y {
                    continue;
                }
                if self.data[self.index(x2, y2)] == Light::On {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn evolve(&mut self) {
        let mut new_grid = vec![Light::Off; self.data.len()];

        for y in 0..self.width {
            for x in 0..self.width {
                let neighbors = self.surrounding_neighbors(x, y);

                let index = self.index(x, y);
                new_grid[index] = self.data[index].evolved(neighbors);
            }
        }

        self.data = new_grid;
    }

    fn count_lights(&self) -> usize {
        self.data.iter().filter(|&&l| l == Light::On).count()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for (i, l) in self.data.iter().enumerate() {
            write!(fmt, "{}", l)?;
            if i % self.width == self.width - 1 {
                writeln!(fmt)?;
            }
        }
        Ok(())
    }
}

#[aoc(day18, part1)]
fn solve_part1(input: &str) -> usize {
    let mut grid = Grid::parse(input);

    for _ in 1..=100 {
        grid.evolve();
    }

    grid.count_lights()
}

#[aoc(day18, part2)]
fn solve_part2(input: &str) -> usize {
    let mut grid = Grid::parse(input);

    let indexes_to_set = vec![
        grid.index(0, 0),
        grid.index(0, 99),
        grid.index(99, 0),
        grid.index(99, 99),
    ];

    for _ in 1..=100 {
        grid.evolve();

        for i in &indexes_to_set {
            grid.data[*i] = Light::On;
        }
    }

    grid.count_lights()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_parse_and_display() {
        let input = r#".#.#.#
...##.
#....#
..#...
#.#..#
####..
"#;

        let grid = Grid::parse(input);

        assert_eq!(format!("{}", grid), input);
    }

    #[test]
    fn grid_evolve() {
        let input = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;

        let mut grid = Grid::parse(input);
        println!("Initial state:\n{}", grid);

        for i in 1..=4 {
            grid.evolve();
            println!("After {} step(s):\n{}", i, grid);
        }

        let expected = r#"......
......
..##..
..##..
......
......
"#;
        assert_eq!(format!("{}", grid), expected);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day18.txt");

        assert_eq!(solve_part1(input), 768);
        assert_eq!(solve_part2(input), 781);
    }
}
