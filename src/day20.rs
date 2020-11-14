use aoc_runner_derive::aoc;

// This gets really expensive with larger numbers
fn presents_delivered(house_number: u32) -> u32 {
    let mut presents = house_number;

    for elf in 1..=house_number / 2 {
        if house_number % elf == 0 {
            presents += elf;
        }
    }

    presents * 10
}

#[aoc(day20, part1, iterate)]
fn solve_part1_iterate(input: &str) -> u32 {
    let input = input.parse::<u32>().unwrap();

    let step_size = 10;
    let mut house_number = input / 50;

    loop {
        // assumption: the desired number will be dividable by 3 and 10
        if house_number % 3 != 0 && house_number % 10 != 0 {
            continue;
        }

        let presents = presents_delivered(house_number);

        // println!("{} | {}", house_number, presents);

        if presents >= input {
            return house_number;
        }

        if presents < input {
            house_number += step_size;
        }
    }
}

#[aoc(day20, part1, array)]
fn solve_part1_array(input: &str) -> u32 {
    let input = input.parse::<u32>().unwrap();

    // assumption: the desired number will be smaller than input / 25
    let max_elf: u32 = input / 25;

    let mut numbers = vec![0; max_elf as usize];

    for elf in 1..max_elf {
        for factor in 1..max_elf / 2 {
            let house = elf * factor;
            if house >= max_elf {
                break;
            }

            numbers[house as usize] += 10 * elf;
        }
    }

    numbers
        .iter()
        .enumerate()
        .find(|(_, &h)| h >= input)
        .map(|(i, _)| i)
        .unwrap() as u32
}

#[aoc(day20, part2)]
fn solve_part2(input: &str) -> u32 {
    let input = input.parse::<u32>().unwrap();

    // assumption: the desired number will be smaller than input / 25
    let max_elf: u32 = input / 25;

    let mut numbers = vec![0; max_elf as usize];

    for elf in 1..max_elf {
        for factor in 1..=50 {
            let house = elf * factor;
            if house >= max_elf {
                break;
            }

            numbers[house as usize] += 11 * elf;
        }
    }

    numbers
        .iter()
        .enumerate()
        .find(|(_, &h)| h >= input)
        .map(|(i, _)| i)
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presents_delivered_test() {
        let examples = vec![
            (1, 10),
            (2, 30),
            (3, 40),
            (4, 70),
            (5, 60),
            (6, 120),
            (7, 80),
            (8, 150),
            (9, 130),
        ];

        for (house_number, presents) in examples {
            assert_eq!(
                presents_delivered(house_number),
                presents,
                "house_number = {}",
                house_number
            );
        }
    }

    #[test]
    #[ignore] // expensive
    fn real_input_part1_iterate() {
        assert_eq!(solve_part1_iterate("36000000"), 831600);
        assert_eq!(solve_part1_iterate("34000000"), 786240);
    }

    #[test]
    fn real_input_part1_array() {
        assert_eq!(solve_part1_array("36000000"), 831600);
        assert_eq!(solve_part1_array("34000000"), 786240);
    }

    #[test]
    fn real_input_part2() {
        assert_eq!(solve_part2("36000000"), 884520);
    }
}
