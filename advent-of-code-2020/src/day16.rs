use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
struct Rule<'a> {
    name: &'a str,
    constraints: ArrayVec<[RangeInclusive<u32>; 2]>,
}

impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Self {
        let (name, constraints) = input.split(": ").collect_tuple().unwrap();

        let constraints = constraints
            .split(" or ")
            .map(|c| serde_scan::scan!("{}-{}" <- c).unwrap())
            .map(|(from, to)| from..=to)
            .collect();

        Self { name, constraints }
    }

    fn check(&self, value: u32) -> bool {
        self.constraints.iter().any(|rule| rule.contains(&value))
    }
}

#[derive(Debug)]
struct Ticket {
    values: ArrayVec<[u32; 20]>,
}

impl Ticket {
    fn parse(input: &str) -> Self {
        Self {
            values: input.split(',').map(|x| x.parse().unwrap()).collect(),
        }
    }

    fn contains_all_valid_values(&self, rules: &[Rule]) -> bool {
        self.values
            .iter()
            .all(|&value| rules.iter().any(|rule| rule.check(value)))
    }

    fn error_rate(&self, rules: &[Rule]) -> u32 {
        self.values
            .iter()
            .filter(|&&value| rules.iter().all(|rule| !rule.check(value)))
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut segements = input.split("\n\n");

    let rules = segements.next().unwrap();
    let rules = rules.lines().map(Rule::parse).collect();

    let my_ticket = segements.next().unwrap();
    let my_ticket = Ticket::parse(my_ticket.lines().nth(1).unwrap());

    let nearby_tickets = segements.next().unwrap();
    let nearby_tickets = nearby_tickets.lines().skip(1).map(Ticket::parse).collect();

    (rules, my_ticket, nearby_tickets)
}

#[aoc(day16, part1)]
fn solve_part1(input: &str) -> u32 {
    let (rules, _, nearby_tickets) = parse_input(input);

    nearby_tickets
        .into_iter()
        .map(|ticket| ticket.error_rate(&rules))
        .sum()
}

#[aoc(day16, part2)]
fn solve_part2(input: &str) -> u64 {
    let (rules, my_ticket, mut nearby_tickets) = parse_input(input);

    // remove tickets with invalid values
    nearby_tickets.retain(|ticket| ticket.contains_all_valid_values(&rules));

    let mut valid_combination = Vec::with_capacity(rules.len());
    find_valid_combination(&nearby_tickets, &mut valid_combination, rules);

    valid_combination
        .iter()
        .zip(my_ticket.values.iter())
        .filter(|(rule, _)| rule.name.starts_with("departure"))
        .map(|(_, &value)| value as u64)
        .product()
}

fn find_valid_combination<'a>(
    tickets: &[Ticket],
    valid_combination_so_far: &mut Vec<Rule<'a>>,
    remaining_rules: Vec<Rule<'a>>,
) -> bool {
    if remaining_rules.len() == 1 {
        valid_combination_so_far.push(remaining_rules[0].clone());
        return true;
    }

    for (i, rule) in remaining_rules.iter().enumerate() {
        if tickets
            .iter()
            .all(|t| rule.check(t.values[valid_combination_so_far.len()]))
        {
            let mut new_remaining = remaining_rules.clone();
            new_remaining.remove(i);

            valid_combination_so_far.push(rule.clone());

            if find_valid_combination(tickets, valid_combination_so_far, new_remaining) {
                return true;
            } else {
                // dead end - remove latest rule again
                valid_combination_so_far.pop();
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        assert_eq!(solve_part1(input), 71);
    }

    #[test]
    fn example_part2() {
        let input = "\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        let (rules, _, nearby_tickets) = parse_input(input);

        let mut valid_combination = Vec::with_capacity(rules.len());
        find_valid_combination(&nearby_tickets, &mut valid_combination, rules);

        assert_eq!(
            valid_combination.iter().map(|r| r.name).collect::<Vec<_>>(),
            vec!["row", "class", "seat"]
        );
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day16.txt");

        assert_eq!(solve_part1(input), 23036);
        assert_eq!(solve_part2(input), 1909224687553);
    }
}
