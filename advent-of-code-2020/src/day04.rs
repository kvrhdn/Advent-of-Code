use aoc_runner_derive::aoc;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, ops::RangeInclusive};

struct Passport<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn parse(data: &'a str) -> Self {
        let data = data
            .split_ascii_whitespace()
            .map(|field| field.split(':').collect_tuple::<(&str, &str)>().unwrap())
            .collect();

        Self { data }
    }

    fn is_valid_v1(&self) -> bool {
        let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        required_fields
            .iter()
            .all(|field| self.data.contains_key(field))
    }

    fn is_valid_v2(&self) -> bool {
        if !self.is_valid_v1() {
            return false;
        }

        let is_number_within = |value: Option<&&str>, range: RangeInclusive<u32>| -> bool {
            value
                .and_then(|value| value.parse::<u32>().ok())
                .filter(|num| range.contains(num))
                .is_some()
        };

        let must_match = move |value: Option<&&str>, r: &Regex| -> bool {
            value.filter(|value| r.is_match(value)).is_some()
        };

        lazy_static! {
            static ref RE_HGT: Regex = Regex::new(r"^(?P<value>\d+)(?P<unit>cm|in)$").unwrap();
            static ref RE_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref RE_ECL: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
            static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        let validate_hgt = |value: Option<&&str>| -> bool {
            value
                .and_then(|value| RE_HGT.captures(value))
                .and_then(|cap| {
                    Some((
                        cap.name("value")?.as_str().parse::<u32>().ok()?,
                        cap.name("unit")?.as_str(),
                    ))
                })
                .filter(|(value, unit)| match *unit {
                    "cm" => (150..=193).contains(value),
                    "in" => (59..=76).contains(value),
                    _ => false,
                })
                .is_some()
        };

        is_number_within(self.data.get("byr"), 1920..=2002)
            && is_number_within(self.data.get("iyr"), 2010..=2020)
            && is_number_within(self.data.get("eyr"), 2020..=2030)
            && validate_hgt(self.data.get("hgt"))
            && must_match(self.data.get("hcl"), &RE_HCL)
            && must_match(self.data.get("ecl"), &RE_ECL)
            && must_match(self.data.get("pid"), &RE_PID)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Passport> {
    input
        .split("\n\n")
        .map(|passport| Passport::parse(passport))
}

#[aoc(day4, part1)]
fn solve_part1(input: &str) -> usize {
    parse_input(input).filter(|p| p.is_valid_v1()).count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> usize {
    parse_input(input).filter(|p| p.is_valid_v2()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(solve_part1(input), 2);
    }

    #[test]
    fn examples_part2() {
        let examples = vec![
            (
                "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
                0,
            ),
            (
                "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
                4,
            ),
        ];

        for (input, expected) in examples {
            assert_eq!(solve_part2(input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day4.txt");

        assert_eq!(solve_part1(input), 250);
        assert_eq!(solve_part2(input), 158);
    }
}
