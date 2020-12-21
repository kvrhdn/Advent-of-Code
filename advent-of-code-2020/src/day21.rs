use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: ArrayVec<[&'a str; 3]>,
}

impl<'a> Food<'a> {
    fn parse(s: &'a str) -> Self {
        let (ingredients, allergens) = s.split_once(" (contains ").unwrap();
        // remove closing bracket ')'
        let allergens = &allergens[..allergens.len() - 1];

        Self {
            ingredients: ingredients.split(' ').collect(),
            allergens: allergens.split(", ").collect(),
        }
    }
}

fn process_input(input: &str) -> Vec<Food> {
    input.lines().map(Food::parse).collect()
}

fn extract_ingredient_allergen_map<'a>(food: &'a [Food]) -> HashMap<&'a str, &'a str> {
    let mut unaccounted_allergens = food
        .iter()
        .flat_map(|f| f.allergens.iter())
        .copied()
        .collect::<HashSet<_>>();

    let mut ingredient_allergen_map = HashMap::new();

    loop {
        for allergen in &unaccounted_allergens {
            let food_with_allergen = food.iter().filter(|f| f.allergens.contains(&allergen));

            let mut common_ingredients = food_with_allergen
                .map(|f| f.ingredients.clone())
                .fold_first(|mut common_ingredients, ingredients| {
                    common_ingredients.retain(|ir| ingredients.contains(ir));
                    common_ingredients
                })
                .unwrap();

            common_ingredients.retain(|ir| !ingredient_allergen_map.contains_key(ir));

            if common_ingredients.len() == 1 {
                ingredient_allergen_map.insert(common_ingredients[0], *allergen);
            }
        }

        unaccounted_allergens.retain(|allergen| {
            ingredient_allergen_map
                .values()
                .filter(|&a| allergen == a)
                .count()
                == 0
        });

        if unaccounted_allergens.is_empty() {
            break;
        }
    }

    ingredient_allergen_map
}

#[aoc(day21, part1)]
fn solve_part1(input: &str) -> usize {
    let food = process_input(input);
    let ingredient_allergen_map = extract_ingredient_allergen_map(&food);

    food.iter()
        .flat_map(|f| f.ingredients.iter())
        .filter(|&ir| !ingredient_allergen_map.contains_key(ir))
        .count()
}

#[aoc(day21, part2)]
fn solve_part2(input: &str) -> String {
    let food = process_input(input);
    let ingredient_allergen_map = extract_ingredient_allergen_map(&food);

    let sorted_ingredients = ingredient_allergen_map
        .into_iter()
        .sorted_by_key(|(_, allergen)| *allergen)
        .map(|(ingredient, _)| ingredient)
        .collect::<Vec<_>>();

    sorted_ingredients.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        assert_eq!(solve_part1(input), 5);
        assert_eq!(solve_part2(input), "mxmxvkd,sqjhc,fvjkl");
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day21.txt");

        assert_eq!(solve_part1(input), 2282);
        assert_eq!(
            solve_part2(input),
            "vrzkz,zjsh,hphcb,mbdksj,vzzxl,ctmzsr,rkzqs,zmhnj"
        );
    }
}
