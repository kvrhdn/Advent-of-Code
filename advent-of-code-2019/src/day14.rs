#![allow(clippy::needless_collect)]

use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[aoc(day14, part1)]
fn solve_part1(input: &str) -> u64 {
    let reactions = parse_input(input);
    let mut spare_ingredients: HashMap<Ingredient, u64> = HashMap::new();

    produce_fuel(1, &mut spare_ingredients, &reactions)
}

#[aoc(day14, part2)]
fn solve_part2(input: &str) -> u64 {
    let reactions = parse_input(input);

    let max_ore: u64 = 1_000_000_000_000;
    let mut ore_left = max_ore;
    let mut fuel_created = 0;

    // preserve spare ingredients across iterations
    let mut spare_ingredients: HashMap<Ingredient, u64> = HashMap::new();

    loop {
        let avg_ore_needed_per_fuel = (max_ore - ore_left) as f32 / fuel_created as f32;

        // the amount of fuel we can create with the remaining ORE, considering
        // the average ORE consumption up to now
        let fuel_step = if avg_ore_needed_per_fuel.is_nan() {
            1
        } else {
            (ore_left as f32 / avg_ore_needed_per_fuel).floor() as u64
        };

        println!(
            "ORE left: {:13}, fuel created: {:9}, will create {:9} fuel next",
            ore_left, fuel_created, fuel_step
        );

        ore_left -= produce_fuel(fuel_step, &mut spare_ingredients, &reactions);

        fuel_created += fuel_step;

        // can't create any fuel anymore (this is not guaranteed to work, but it
        // works with the examples and input)
        if ore_left < avg_ore_needed_per_fuel as u64 {
            return fuel_created;
        }
    }
}

fn produce_fuel<'a>(
    fuel: u64,
    spare_ingredients: &mut HashMap<Ingredient<'a>, u64>,
    reactions: &[Reaction<'a>],
) -> u64 {
    let mut needed_ingredients: HashMap<Ingredient, u64> = HashMap::new();
    needed_ingredients.insert("FUEL", fuel);

    // loop until the only remaining needed ingredient is ORE
    while needed_ingredients
        .iter()
        .any(|(&ingredient, &count)| ingredient != "ORE" && count > 0)
    {
        let ingredients = needed_ingredients
            .keys()
            .copied()
            .collect::<Vec<Ingredient>>();

        for needed in ingredients.into_iter() {
            if needed == "ORE" {
                continue;
            }

            let mut needed_count = *needed_ingredients.get(needed).unwrap();

            // remove from needed
            needed_ingredients.remove_entry(needed);

            let recipe = reactions.iter().find(|r| r.output.1 == needed).unwrap();

            // use from spare if available
            if let Some(spare_count) = spare_ingredients.get_mut(needed) {
                if *spare_count >= needed_count {
                    *spare_count -= needed_count;
                    continue;
                } else {
                    needed_count -= *spare_count;
                    *spare_count = 0;
                }
            }

            let recipe_count = {
                let mut recipe_count = 1;
                while recipe_count * recipe.output.0 < needed_count {
                    recipe_count += 1;
                }
                recipe_count
            };

            // put excess in spare
            let spare_count = (recipe.output.0 * recipe_count) - needed_count;
            if spare_count > 0 {
                let count = spare_ingredients.entry(needed).or_default();
                *count += spare_count;
            }

            for (input_count, input) in recipe.inputs.iter() {
                let count = needed_ingredients.entry(input).or_default();
                *count += recipe_count * input_count;
            }
        }
    }

    return *needed_ingredients.get("ORE").unwrap();
}

type Ingredient<'a> = &'a str;

type IngredientQuantity<'a> = (u64, Ingredient<'a>);

#[derive(Debug)]
struct Reaction<'a> {
    inputs: Vec<IngredientQuantity<'a>>,
    output: IngredientQuantity<'a>,
}

fn parse_input(input: &str) -> Vec<Reaction> {
    input.lines().map(|line| parse_reaction(line)).collect()
}

fn parse_reaction(line: &str) -> Reaction {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
    }

    let ingredients: Vec<IngredientQuantity> = RE
        .captures_iter(line)
        .map(|capture| {
            let quantity: u64 = capture.get(1).unwrap().as_str().parse().unwrap();
            let ingredient = capture.get(2).unwrap().as_str();

            (quantity, ingredient)
        })
        .collect();

    let (output, inputs) = ingredients.split_last().unwrap();

    Reaction {
        inputs: inputs.to_vec(),
        output: *output,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let examples = vec![
            (
                r#"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"#,
                31,
                None,
            ),
            (
                r#"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"#,
                165,
                None,
            ),
            (
                r#"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#,
                13312,
                Some(82892753),
            ),
            (
                r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"#,
                180697,
                Some(5586022),
            ),
            (
                r#"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"#,
                2210736,
                Some(460664),
            ),
        ];
        for (input, expected_part1, expected_part2) in examples {
            assert_eq!(solve_part1(input), expected_part1);

            if let Some(expected_part2) = expected_part2 {
                assert_eq!(solve_part2(input), expected_part2);
            }
        }
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2019/day14.txt");

        assert_eq!(solve_part1(input), 97422);
        assert_eq!(solve_part2(input), 13108426);
    }
}
