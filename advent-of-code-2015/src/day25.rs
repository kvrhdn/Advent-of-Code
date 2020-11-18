use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

struct DiagonalIterator {
    row: usize,
    col: usize,
}

impl DiagonalIterator {
    fn from(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Iterator for DiagonalIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == 1 {
            self.row = self.col + 1;
            self.col = 1;
        } else {
            self.row -= 1;
            self.col += 1;
        }

        Some((self.row, self.col))
    }
}

#[aoc_generator(day25, part1)]
fn parse_input(input: &str) -> (usize, usize) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^To continue, please consult the code grid in the manual.  Enter the code at row (\d+), column (\d+).$").unwrap();
    }
    let captures = RE.captures(input).unwrap();

    let row = captures.get(1).unwrap().as_str().parse().unwrap();
    let col = captures.get(2).unwrap().as_str().parse().unwrap();

    (row, col)
}

#[aoc(day25, part1)]
fn solve_part1(input_position: &(usize, usize)) -> u64 {
    let mut value = 20151125;

    for position in DiagonalIterator::from(1, 1) {
        value = (value * 252533) % 33554393;

        if position == *input_position {
            break;
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagonal_iterator() {
        let mut diagonal_iterator = DiagonalIterator::from(1, 1);

        assert_eq!(diagonal_iterator.next(), Some((2, 1)));
        assert_eq!(diagonal_iterator.next(), Some((1, 2)));
        assert_eq!(diagonal_iterator.next(), Some((3, 1)));
        assert_eq!(diagonal_iterator.next(), Some((2, 2)));
        assert_eq!(diagonal_iterator.next(), Some((1, 3)));
    }

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&(2, 1)), 31916031);
        assert_eq!(solve_part1(&(6, 6)), 27995004);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day25.txt").trim_end();

        let parsed_input = parse_input(input);

        assert_eq!(solve_part1(&parsed_input), 8997277);
    }
}
