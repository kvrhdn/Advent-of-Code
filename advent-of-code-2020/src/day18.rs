use aoc_runner_derive::aoc;
use lexical::parse_partial;

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Mult,
}

fn solve_expression(expr: &str, plus_has_precedence: bool) -> (u64, usize) {
    let mut acc = 0;
    let mut op: Option<Operator> = None;

    let mut i = 0;
    let len = expr.len();

    let apply = |acc: &mut u64, value, op: Option<Operator>| match op {
        Some(Operator::Plus) => *acc += value,
        Some(Operator::Mult) => *acc *= value,
        None => *acc = value,
    };

    while i < len {
        match expr.as_bytes()[i] {
            b' ' => {
                i += 1;
            }
            b'+' => {
                op = Some(Operator::Plus);
                i += 1;
            }
            b'*' => {
                op = Some(Operator::Mult);
                i += 1;

                if plus_has_precedence {
                    // if plus has precedence, we calculate everything between
                    // multiplications first (this works since there are only
                    // two operators)
                    let (value, n) = solve_expression(&expr[i..], plus_has_precedence);
                    i += n;

                    apply(&mut acc, value, op);
                }
            }
            b'(' => {
                let (value, n) = solve_expression(&expr[i + 1..], plus_has_precedence);
                // +2 to skip both braces '(' and ')'
                i += n + 2;

                apply(&mut acc, value, op);
            }
            b')' => return (acc, i),
            n if (b'0'..=b'9').contains(&n) => {
                let (value, n): (u64, usize) = parse_partial(&expr[i..]).unwrap();
                i += n;

                apply(&mut acc, value, op);
            }
            _ => panic!("unsupported char {}", expr),
        }
    }

    (acc, i)
}

#[aoc(day18, part1)]
fn solve_part1(input: &str) -> u64 {
    input.lines().map(|l| solve_expression(l, false).0).sum()
}

#[aoc(day18, part2)]
fn solve_part2(input: &str) -> u64 {
    input.lines().map(|l| solve_expression(l, true).0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let examples = vec![
            ("1 + 2 * 3 + 4 * 5 + 6", 71, 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51, 51),
            ("2 * 3 + (4 * 5)", 26, 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437, 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240, 669060),
            (
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                13632,
                23340,
            ),
        ];

        for (expression, expected_part1, expected_part2) in examples {
            assert_eq!(solve_expression(expression, false).0, expected_part1);
            assert_eq!(solve_expression(expression, true).0, expected_part2);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day18.txt");

        assert_eq!(solve_part1(input), 1451467526514);
        assert_eq!(solve_part2(input), 224973686321527);
    }
}
