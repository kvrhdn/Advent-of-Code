use std::collections::HashMap;

use aoc_runner_derive::aoc;
use serde_scan::scan;

// Rule IDs are never higher than about 140
type RuleId = u8;

type RuleMap = HashMap<RuleId, Rule>;

#[derive(Debug)]
enum Rule {
    Literal(u8),
    And(Vec<RuleId>),
    Or(Vec<RuleId>, Vec<RuleId>),
    Rule8(),
    Rule11(),
}

impl Rule {
    fn parse(s: &str) -> Rule {
        if let Ok(c) = scan!("\"{}\"" <- s) {
            let c: char = c;
            return Rule::Literal(c as u8);
        }

        if s.contains('|') {
            if let Ok((r1, r2, r3, r4)) = scan!("{} {} | {} {}" <- s) {
                return Rule::Or(vec![r1, r2], vec![r3, r4]);
            }
            if let Ok((r1, r2)) = scan!("{} | {}" <- s) {
                return Rule::Or(vec![r1], vec![r2]);
            }
        }

        let rule_ids = s
            .split(' ')
            .map(|s| s.parse::<RuleId>().unwrap())
            .collect::<Vec<_>>();

        Rule::And(rule_ids)
    }

    /// Evalutate checks whether the given string matches with the rule and
    /// returns the amount of characters 'consumed' by the rule. If the rule
    /// contains a loop (only Rule8 or Rule11) it can return multiple matches.
    fn evaluate(&self, rules: &RuleMap, s: &str) -> Vec<usize> {
        if s.is_empty() {
            return vec![];
        }

        match self {
            Rule::Literal(c) => {
                if s.as_bytes()[0] == *c {
                    vec![1]
                } else {
                    vec![]
                }
            }
            Rule::And(rs) => {
                let mut possible_n = vec![0];

                for rule in rs.iter().map(|id| rules.get(id).unwrap()) {
                    // for every possible n, evaluate the rule and collect the new valid n's
                    possible_n = possible_n
                        .into_iter()
                        .flat_map(|n| {
                            rule.evaluate(rules, &s[n..])
                                .into_iter()
                                .map(move |result| n + result)
                        })
                        .collect();

                    // there were no matches
                    if possible_n.is_empty() {
                        return possible_n;
                    }
                }

                possible_n
            }
            Rule::Or(rs1, rs2) => {
                let n = Rule::And(rs1.clone()).evaluate(rules, s);
                if !n.is_empty() {
                    return n;
                }
                Rule::And(rs2.clone()).evaluate(rules, s)
            }
            Rule::Rule8() => {
                // 8: 42 | 42 8
                let rule_42 = rules.get(&42).unwrap();

                // there should always be at least one successful evaluation of rule 42
                let mut possible_n = rule_42.evaluate(rules, s);
                if possible_n.is_empty() {
                    return possible_n;
                }

                let mut new = possible_n.clone();

                loop {
                    new = new
                        .into_iter()
                        .flat_map(|n| {
                            rule_42
                                .evaluate(rules, &s[n..])
                                .into_iter()
                                .map(move |result| n + result)
                        })
                        .collect();

                    // no new matches - return matches so far
                    if new.is_empty() {
                        return possible_n;
                    }

                    possible_n.extend(new.iter());
                }
            }
            Rule::Rule11() => {
                // 11: 42 31 | 42 11 31
                let rule_42 = rules.get(&42).unwrap();
                let rule_31 = rules.get(&31).unwrap();

                // there should always be at least one successful evaluation of rule 42
                let mut possible_n = rule_42.evaluate(rules, s);
                if possible_n.is_empty() {
                    return possible_n;
                }

                // there can be many evaluations of rule 11
                let mut new = possible_n.clone();

                loop {
                    new = new
                        .into_iter()
                        .flat_map(|n| {
                            Rule::Rule11()
                                .evaluate(rules, &s[n..])
                                .into_iter()
                                .map(move |result| n + result)
                        })
                        .collect();

                    // no new matches
                    if new.is_empty() {
                        break;
                    }

                    possible_n.extend(new.iter());
                }

                // there should be a final successful evaluation of rule 31
                possible_n
                    .into_iter()
                    .flat_map(|n| {
                        rule_31
                            .evaluate(rules, &s[n..])
                            .into_iter()
                            .map(move |result| n + result)
                    })
                    .collect()
            }
        }
    }
}

fn parse_input(input: &str) -> (RuleMap, impl Iterator<Item = &str>) {
    let (rules, messages) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| scan!("{}: {}" <- line).unwrap())
        .map(|(id, expr)| (id, Rule::parse(expr)))
        .collect();

    (rules, messages.lines())
}

#[aoc(day19, part1)]
fn solve_part1(input: &str) -> usize {
    let (rules, messages) = parse_input(input);

    let rule_0 = rules.get(&0).unwrap();

    messages
        .filter(|m| rule_0.evaluate(&rules, m).contains(&m.len()))
        .count()
}

#[aoc(day19, part2)]
fn solve_part2(input: &str) -> usize {
    let (mut rules, messages) = parse_input(input);

    rules.insert(8, Rule::Rule8());
    rules.insert(11, Rule::Rule11());

    let rule_0 = rules.get(&0).unwrap();

    messages
        .filter(|m| rule_0.evaluate(&rules, m).contains(&m.len()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

        assert_eq!(solve_part1(input), 2);
    }

    #[test]
    fn example_part2() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        assert_eq!(solve_part1(input), 3);
        assert_eq!(solve_part2(input), 12);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day19.txt");

        assert_eq!(solve_part1(input), 187);
        assert_eq!(solve_part2(input), 392);
    }
}
