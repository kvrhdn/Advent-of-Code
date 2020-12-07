use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Bag<'a> = &'a str;

struct Rule<'a> {
    bag: Bag<'a>,
    contains: HashMap<Bag<'a>, u32>,
}

impl<'a> Rule<'a> {
    fn parse(input: &str) -> Rule {
        let (bag, contains) = input.split(" bags contain ").collect_tuple().unwrap();

        let contains = if contains != "no other bags." {
            contains
                .split(", ")
                .map(|bag| {
                    // assumption: count is always a single digit number
                    let count = bag[0..1].parse::<u32>().unwrap();

                    let bags = bag.find(" bag").unwrap();
                    let name = &bag[2..bags];

                    (name, count)
                })
                .collect::<HashMap<_, _>>()
        } else {
            HashMap::new()
        };

        Rule { bag, contains }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Rule> {
    input.lines().map(|line| Rule::parse(line))
}

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> usize {
    let rules = parse_input(input).collect::<Vec<_>>();

    let mut bags_that_contain_shiny_gold = HashSet::new();

    let mut bags = HashSet::new();
    bags.insert("shiny gold");

    loop {
        let mut new_bags = HashSet::new();

        for bag in bags.iter() {
            for r in rules.iter() {
                if r.contains.contains_key(*bag) {
                    let is_new = bags_that_contain_shiny_gold.insert(r.bag);
                    if is_new {
                        new_bags.insert(r.bag);
                    }
                }
            }
        }

        if new_bags.is_empty() {
            break;
        }

        bags = new_bags;
    }

    bags_that_contain_shiny_gold.len()
}

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> u32 {
    let rules = parse_input(input)
        .map(|rule| (rule.bag, rule))
        .collect::<HashMap<_, _>>();

    let mut shiny_gold_contains = 0;

    let mut bags = HashMap::new();
    bags.insert(&"shiny gold", 1);

    while !bags.is_empty() {
        let mut new_bags = HashMap::new();

        for (&&bag, &count) in bags.iter() {
            if let Some(rule) = rules.get(bag) {
                for (contains, &contains_count) in &rule.contains {
                    let new_bag_count = count * contains_count;

                    let curr_count = new_bags.entry(contains).or_default();
                    *curr_count += new_bag_count;

                    shiny_gold_contains += new_bag_count;
                }
            }
        }

        bags = new_bags;
    }

    shiny_gold_contains
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.Î
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(solve_part1(input), 4);
        assert_eq!(solve_part2(input), 32);
    }

    #[test]
    fn example_part2() {
        let input = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(solve_part2(input), 126);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day7.txt");

        assert_eq!(solve_part1(input), 268);
        assert_eq!(solve_part2(input), 7867);
    }
}
