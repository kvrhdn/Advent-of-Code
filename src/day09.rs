use aoc_runner_derive::{aoc, aoc_generator};
use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Connection {
    from: String,
    to: String,
    distance: u32,
}

impl Connection {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+) to (.+) = (\d+)$").unwrap();
        }
        let captures = RE.captures(s).unwrap().unwrap();

        assert_eq!(captures.len(), 4, "Input: {}", s);

        Connection {
            from: captures.get(1).unwrap().as_str().to_string(),
            to: captures.get(2).unwrap().as_str().to_string(),
            distance: captures.get(3).unwrap().as_str().parse().unwrap(),
        }
    }
}

fn extract_cities(connections: &[Connection]) -> Vec<String> {
    let mut hash_map = HashSet::new();

    for c in connections {
        hash_map.insert(&c.to);
        hash_map.insert(&c.from);
    }

    hash_map.iter().map(|&v| v.clone()).collect()
}

fn find_distance_between(connections: &[Connection], from: &str, to: &str) -> Option<u32> {
    connections
        .iter()
        .find(|&c| (c.from == from && c.to == to) || (c.to == from && c.from == to))
        .map(|c| c.distance)
}

#[derive(Debug)]
struct Route {
    cities: Vec<String>,
}

impl Route {
    fn new() -> Self {
        Self { cities: Vec::new() }
    }

    fn new_with(&self, city: &str) -> Self {
        Self {
            cities: {
                let mut new_cities = self.cities.clone();
                new_cities.push(city.to_string());
                new_cities
            },
        }
    }

    fn total_distance(&self, connections: &[Connection]) -> u32 {
        self.cities
            .windows(2)
            .map(|window| find_distance_between(connections, &window[0], &window[1]).unwrap())
            .sum()
    }
}

fn extract_routes(route: &Route, cities: &[String]) -> Vec<Route> {
    let mut routes = Vec::new();

    for city in cities {
        let route = route.new_with(city);

        let remaining_cities = {
            let mut remaining_cities = cities.to_vec();
            remaining_cities.retain(|v| city != v);
            remaining_cities
        };

        if remaining_cities.is_empty() {
            return vec![route];
        }

        routes.append(&mut extract_routes(&route, &remaining_cities));
    }

    routes
}

#[aoc_generator(day9)]
fn parse_routes(input: &str) -> (Vec<Connection>, Vec<Route>) {
    let connections = input
        .lines()
        .map(|l| Connection::parse(l))
        .collect::<Vec<_>>();
    let cities = extract_cities(&connections);
    let routes = extract_routes(&Route::new(), &cities);

    (connections, routes)
}

#[aoc(day9, part1)]
fn solve_part1(input: &(Vec<Connection>, Vec<Route>)) -> u32 {
    let (connections, routes) = input;
    routes
        .iter()
        .map(|route| route.total_distance(connections))
        .min()
        .unwrap()
}

#[aoc(day9, part2)]
fn solve_part2(input: &(Vec<Connection>, Vec<Route>)) -> u32 {
    let (connections, routes) = input;
    routes
        .iter()
        .map(|route| route.total_distance(connections))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day09::*;

    #[test]
    fn connection_parse() {
        #[rustfmt::skip]
        let examples = vec![
            (
                "London to Dublin = 464",
                Connection { from: "London".into(), to: "Dublin".into(), distance: 464 },
            ),
            (
                "London to Belfast = 518",
                Connection { from: "London".into(), to: "Belfast".into(), distance: 518 },
            ),
            (
                "Dublin to Belfast = 1",
                Connection { from: "Dublin".into(), to: "Belfast".into(), distance: 1 },
            ),
        ];

        for (input, ir) in examples {
            assert_eq!(Connection::parse(input), ir);
        }
    }

    #[test]
    fn examples() {
        let input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

        let r = parse_routes(input);

        assert_eq!(solve_part1(&r), 605);
        assert_eq!(solve_part2(&r), 982);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day9.txt");

        let r = parse_routes(input);

        assert_eq!(solve_part1(&r), 207);
        assert_eq!(solve_part2(&r), 804);
    }
}
