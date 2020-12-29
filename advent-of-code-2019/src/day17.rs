use crate::intcode::{self, Computer};
use aoc_runner_derive::aoc;

#[aoc(day17, part1)]
fn solve_part1(input: &str) -> usize {
    let mut ascii = Computer::new_from_input(input).unwrap();

    ascii.run().unwrap();

    let buffer = ascii.get_output_buffer();

    // for &value in buffer {
    //     print!("{}", value as u8 as char);
    // }

    // find first newline, this is the width of the grid (assuming all lines have the same width)
    let (width, _) = buffer
        .iter()
        .enumerate()
        .find(|(_, &c)| c as u8 == b'\n')
        .unwrap();
    let width = width + 1;

    let mut count = 0;

    for (i, &c) in buffer.iter().enumerate() {
        if c as u8 != b'#' {
            continue;
        }

        if i < width {
            continue;
        }

        if buffer[i - width] == b'#' as i64
            && buffer[i - 1] == b'#' as i64
            && buffer[i + 1] == b'#' as i64
            && buffer[i + width] == b'#' as i64
        {
            count += (i / width) * (i % width);
        }
    }

    count
}

#[aoc(day17, part2)]
fn solve_part2(input: &str) -> i64 {
    /*
    Map:

    ......#############...........................#############
    ......#...........#...........................#...........#
    ......#...........#...........................#...........#
    ......#...........#...........................#...........#
    ......#...........#...........................#...........#
    ......#...........#...........................#...........#
    ......#...........#.#########.........###########.........#
    ......#...........#.#.......#.........#.......#.#.........#
    ......#...........#############.......#.......#.#.........#
    ......#.............#.......#.#.......#.......#.#.........#
    ......#.............#.......#.#.....###########.#.........#
    ......#.............#.......#.#.....#.#.........#.........#
    ......###########...#.......#.#.....#.#.........#.#########
    ................#...#.......#.#.....#.#.........#.#........
    ................#...#.......#############.......#.#........
    ................#...#.........#.....#.#.#.......#.#........
    ................#...#.........#########.#.......###########
    ................#...#...............#...#.........#.......#
    ............^########...............#...#.........#.......#
    ................#...................#...#.........#.......#
    ................#...................###########...#.......#
    ................#.......................#.....#...#.......#
    ........#########.......................###########.......#
    ........#.....................................#...........#
    ........#.....................................#...........#
    ........#.....................................#...........#
    ........#.....................................#...........#
    ........#.....................................#...........#
    ........#.....................................#############
    ........#..................................................
    ........#..................................................
    ........#..................................................
    ........#..................................................
    ........#..................................................
    #########..................................................

    Entire routine:

    A: R8 L12 R8
    A: R8 L12 R8
    B: L10 L10 R8
    C: L12 L12 L10 R10
    B: L10 L10 R8
    C: L12 L12 L10 R10
    B: L10 L10 R8
    A: R8 L12 R8
    C: L12 L12 L10 R10
    A: R8 L12 R8
    */
    let mut input_string = String::new();

    // Main
    input_string += "A,A,B,C,B,C,B,A,C,A\n";
    // Function A
    input_string += "R,8,L,12,R,8\n";
    // Function B
    input_string += "L,10,L,10,R,8\n";
    // Function C
    input_string += "L,12,L,12,L,10,R,10\n";
    // Continuous video feed?
    input_string += "n\n";

    let mut program = intcode::load_program(input).unwrap();
    program[0] = 2;

    let mut ascii = Computer::new(program);

    // feed input
    for &c in input_string.as_bytes() {
        ascii.put_input(c as i64)
    }

    ascii.run().unwrap();

    for &value in ascii.get_output_buffer() {
        // the program will output ASCII until it outputs the actual answer
        if value <= 127 {
            // print!("{}", value as u8 as char);
        } else {
            return value;
        }
    }

    panic!("Program did not finish as expected");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day17.txt");

        assert_eq!(solve_part1(input), 5972);
        assert_eq!(solve_part2(input), 933214);
    }
}
