use aoc_runner_derive::aoc;
use json::JsonValue;
use lazy_static::lazy_static;
use regex::Regex;

#[aoc(day12, part1, regex)]
fn solve_part1_regex(input: &str) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?\d+)").unwrap();
    }

    RE.find_iter(input)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .sum()
}

fn sum_numbers(value: &JsonValue, skip_red: bool) -> i32 {
    match value {
        JsonValue::Null => 0,
        JsonValue::Short(s) => s.parse().unwrap_or_default(),
        JsonValue::String(s) => s.parse().unwrap_or_default(),
        JsonValue::Number(n) => f32::from(*n) as i32,
        JsonValue::Boolean(_) => 0,
        JsonValue::Object(obj) => {
            if skip_red && obj.iter().any(|(_, value)| *value == "red") {
                return 0;
            }
            obj.iter()
                .map(|(_, value)| sum_numbers(value, skip_red))
                .sum()
        }
        JsonValue::Array(values) => values
            .iter()
            .map(|value| sum_numbers(value, skip_red))
            .sum(),
    }
}

#[aoc(day12, part1, json)]
fn solve_part1_json(input: &str) -> i32 {
    let json_value = json::parse(input).unwrap();

    sum_numbers(&json_value, false)
}

#[aoc(day12, part2)]
fn solve_part2(input: &str) -> i32 {
    let json_value = json::parse(input).unwrap();

    sum_numbers(&json_value, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        let examples = vec![
            (r#"[1,2,3]"#, 6),
            (r#"{"a":2,"b":4}"#, 6),
            (r#"[[[3]]]"#, 3),
            (r#"{"a":{"b":4},"c":-1}"#, 3),
            (r#"{"a":[-1,1]}"#, 0),
            (r#"[-1,{"a":1}]"#, 0),
            (r#"[]"#, 0),
            (r#"{}"#, 0),
        ];

        for (input, expected) in examples {
            assert_eq!(solve_part1_regex(input), expected);
            assert_eq!(solve_part1_json(input), expected);
        }
    }

    #[test]
    fn examples_part2() {
        let examples = vec![
            (r#"[1,2,3]"#, 6),
            (r#"[1,{"c":"red","b":2},3]"#, 4),
            (r#"[1,{"a":5,"c":{"c":"red","b":2}},3]"#, 9),
            (r#"[1,{"b":2,"c":"red"},3]"#, 4),
            (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
            (r#"[1,"red",5]"#, 6),
        ];

        for (input, expected) in examples {
            assert_eq!(solve_part2(input), expected);
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day12.txt").trim_end();

        assert_eq!(solve_part1_regex(input), 119433);
        assert_eq!(solve_part1_json(input), 119433);
        assert_eq!(solve_part2(input), 68466);
    }
}
