use std::cmp;

use aoc_runner_derive::aoc;
use itertools::Itertools;

fn weight(packages: &[u32]) -> u32 {
    packages.iter().sum()
}

fn weight_ref(packages: &[&u32]) -> u32 {
    packages.iter().map(|&&p| p).sum()
}

fn quantum_entanglement(packages: &[&u32]) -> u64 {
    // widen type to avoid overflow
    packages.iter().fold(1u64, |a, &b| a * (*b as u64))
}

fn find_quantum_entanglement_of_best_group(packages: &[u32], amount_of_groups: u32) -> u64 {
    let ideal_weight = weight(packages) / amount_of_groups;

    let mut best_quantum_entanglement = u64::MAX;

    for length in 1..=packages.len() {
        // for all combinations of length that have weight == ideal_weight
        for combination in packages
            .iter()
            .permutations(length)
            .filter(|packages| weight_ref(packages) == ideal_weight)
        {
            best_quantum_entanglement = cmp::min(
                quantum_entanglement(&combination),
                best_quantum_entanglement,
            );
        }

        println!(
            "Len: {}, best so far: {}",
            length, best_quantum_entanglement
        );

        // if there has been any successful combination, stop looping since
        // next loop will have a larger length
        if best_quantum_entanglement != u64::MAX {
            break;
        }
    }

    best_quantum_entanglement
}

#[aoc(day24, part1)]
fn solve_part1(input: &str) -> u64 {
    let packages = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    find_quantum_entanglement_of_best_group(&packages, 3)
}

#[aoc(day24, part2)]
fn solve_part2(input: &str) -> u64 {
    let packages = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    find_quantum_entanglement_of_best_group(&packages, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantum_entanglement_examples() {
        assert_eq!(quantum_entanglement(&[&11, &9]), 99);
        assert_eq!(quantum_entanglement(&[&10, &9, &1]), 90);
        assert_eq!(quantum_entanglement(&[&10, &8, &2]), 160);
    }

    #[test]
    fn examples() {
        let packages = &[1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

        assert_eq!(find_quantum_entanglement_of_best_group(packages, 3), 99);
        assert_eq!(find_quantum_entanglement_of_best_group(packages, 4), 44);
    }

    #[test]
    #[ignore = "expensive"]
    fn real_input() {
        let input = include_str!("../input/2015/day24.txt");

        assert_eq!(solve_part1(input), 11846773891);
        assert_eq!(solve_part2(input), 80393059);
    }
}
