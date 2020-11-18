use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<name>\w+): capacity (?P<cap>-?\d+), durability (?P<dur>-?\d+), flavor (?P<fla>-?\d+), texture (?P<tex>-?\d+), calories (?P<cal>-?\d+)$")
                    .unwrap();
        }
        let captures = RE.captures(s).unwrap();

        Ingredient {
            name: captures.name("name").unwrap().as_str().to_string(),
            capacity: captures.name("cap").unwrap().as_str().parse().unwrap(),
            durability: captures.name("dur").unwrap().as_str().parse().unwrap(),
            flavor: captures.name("fla").unwrap().as_str().parse().unwrap(),
            texture: captures.name("tex").unwrap().as_str().parse().unwrap(),
            calories: captures.name("cal").unwrap().as_str().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct IngredientAccumulator {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl IngredientAccumulator {
    fn from_ingredients(ingredients: &[&Ingredient]) -> Self {
        ingredients
            .iter()
            .fold(Self::new(), |acc, ir| acc.with_ingredient(ir))
    }

    fn new() -> Self {
        Self {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }

    fn with_ingredient(&self, ingredient: &Ingredient) -> Self {
        Self {
            capacity: self.capacity + ingredient.capacity,
            durability: self.durability + ingredient.durability,
            flavor: self.flavor + ingredient.flavor,
            texture: self.texture + ingredient.texture,
            calories: self.calories + ingredient.calories,
        }
    }

    fn total_score(&self) -> u32 {
        value_or_zero(self.capacity)
            * value_or_zero(self.durability)
            * value_or_zero(self.flavor)
            * value_or_zero(self.texture)
    }

    fn calories(&self) -> i32 {
        self.calories
    }
}

fn value_or_zero(value: i32) -> u32 {
    if value < 0 {
        0
    } else {
        value as u32
    }
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<Ingredient> {
    input.lines().map(|l| Ingredient::parse(l)).collect()
}

fn ingredient_accumulator_for_combinations<'a>(
    ingredients: &'a [Ingredient],
    length_of_combinations: usize,
) -> impl Iterator<Item = IngredientAccumulator> + 'a {
    ingredients
        .iter()
        .combinations_with_replacement(length_of_combinations)
        .map(|irs| IngredientAccumulator::from_ingredients(&irs))
}

#[aoc(day15, part1)]
fn solve_part1(ingredients: &[Ingredient]) -> u32 {
    ingredient_accumulator_for_combinations(ingredients, 100)
        .map(|acc| acc.total_score())
        .max()
        .unwrap()
}

#[aoc(day15, part2)]
fn solve_part2(ingredients: &[Ingredient]) -> u32 {
    ingredient_accumulator_for_combinations(ingredients, 100)
        .filter(|acc| acc.calories() == 500)
        .map(|acc| acc.total_score())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ingredients() {
        let examples = vec![
            (
                "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
                Ingredient {
                    name: "Butterscotch".into(),
                    capacity: -1,
                    durability: -2,
                    flavor: 6,
                    texture: 3,
                    calories: 8,
                },
            ),
            (
                "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
                Ingredient {
                    name: "Cinnamon".into(),
                    capacity: 2,
                    durability: 3,
                    flavor: -2,
                    texture: -1,
                    calories: 3,
                },
            ),
        ];

        for (input, ingredient) in examples {
            assert_eq!(Ingredient::parse(input), ingredient);
        }
    }

    #[test]
    fn examples() {
        let input = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#;

        let ingredients = parse_input(input);

        assert_eq!(solve_part1(&ingredients), 62842880);
        assert_eq!(solve_part2(&ingredients), 57600000);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2015/day15.txt");

        let reindeer = parse_input(input);

        assert_eq!(solve_part1(&reindeer), 18965440);
        assert_eq!(solve_part2(&reindeer), 15862900);
    }
}
