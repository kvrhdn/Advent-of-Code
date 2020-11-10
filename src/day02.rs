use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp;

#[derive(Debug, Eq, PartialEq)]
pub struct Gift {
    w: u32,
    h: u32,
    l: u32,
}

impl Gift {
    fn parse(s: &str) -> Gift {
        let mut iter = s.split('x').map(|n| n.parse().unwrap());
        Gift {
            w: iter.next().unwrap(),
            h: iter.next().unwrap(),
            l: iter.next().unwrap(),
        }
    }

    fn required_wrapping_paper(&self) -> u32 {
        let lw = self.l * self.w;
        let wh = self.w * self.h;
        let hl = self.h * self.l;

        (2 * lw) + (2 * wh) + (2 * hl) + min(lw, wh, hl)
    }

    fn required_ribbon(&self) -> u32 {
        let mut sides = vec![self.w, self.h, self.l];
        sides.sort_unstable();
        let smallest_perimeter = (2 * sides[0]) + (2 * sides[1]);

        let volume = self.w * self.h * self.l;
        smallest_perimeter + volume
    }
}

fn min(d1: u32, d2: u32, d3: u32) -> u32 {
    cmp::min(d1, cmp::min(d2, d3))
}

#[aoc_generator(day2)]
pub fn parse_gifts(input: &str) -> Vec<Gift> {
    input.lines().map(|l| Gift::parse(l)).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(gifts: &[Gift]) -> u32 {
    gifts.iter().map(|g| g.required_wrapping_paper()).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(gifts: &[Gift]) -> u32 {
    gifts.iter().map(|g| g.required_ribbon()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_parse_gifts() {
        assert_eq!(
            parse_gifts("2x3x4\n1x1x10"),
            vec![Gift { w: 2, h: 3, l: 4 }, Gift { w: 1, h: 1, l: 10 }]
        );
    }

    #[test]
    fn required_wrapping_paper() {
        let gift = Gift { w: 2, h: 3, l: 4 };
        assert_eq!(gift.required_wrapping_paper(), 58);

        let gift = Gift { w: 1, h: 1, l: 10 };
        assert_eq!(gift.required_wrapping_paper(), 43);
    }

    #[test]
    fn required_ribbon() {
        let gift = Gift { w: 2, h: 3, l: 4 };
        assert_eq!(gift.required_ribbon(), 34);

        let gift = Gift { w: 1, h: 1, l: 10 };
        assert_eq!(gift.required_ribbon(), 14);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day2.txt");

        let gifts = parse_gifts(input);

        assert_eq!(solve_part1(&gifts), 1606483);
        assert_eq!(solve_part2(&gifts), 3842356);
    }
}
