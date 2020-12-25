#![feature(iterator_fold_self)]
#[macro_use]
extern crate lazy_static;

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{bail, Result};
use itertools::Itertools;
use regex::Regex;
use utils::{read_lines, InputParseError};

struct FoodItem {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for FoodItem {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref FOOD_RE: Regex = Regex::new(r"^((\w+ )+)\(contains(( \w+,?)+)\)$").unwrap();
        }
        fn set_from_string(string: &str) -> HashSet<String> {
            string
                .trim()
                .split_whitespace()
                .map(|s| s.trim_matches(|c: char| c.is_whitespace() || c == ','))
                .map(str::to_string)
                .collect()
        }
        let (ingredients, allergens) = FOOD_RE
            .captures(s)
            .and_then(|cs| cs.get(1).zip(cs.get(3)))
            .map(|(ingr, aller)| {
                (
                    set_from_string(ingr.as_str()),
                    set_from_string(aller.as_str()),
                )
            })
            .ok_or(InputParseError)?;
        Ok(FoodItem {
            ingredients,
            allergens,
        })
    }
}

fn which_ingredients_might_contain_allergen(allergen: &str, foods: &[FoodItem]) -> HashSet<String> {
    foods
        .iter()
        .filter(|f| f.allergens.contains(allergen))
        .map(|f| f.ingredients.clone())
        .fold_first(|acc, ingrs| acc.intersection(&ingrs).cloned().collect())
        .unwrap_or_default()
}

fn part1(foods: &[FoodItem]) -> usize {
    let all_allergens: HashSet<_> = foods.iter().flat_map(|f| f.allergens.iter()).collect();

    let allergen_containing_ingredients = all_allergens
        .iter()
        .map(|&aller| which_ingredients_might_contain_allergen(aller, foods))
        .fold(HashSet::new(), |acc, ingrs| {
            acc.union(&ingrs).cloned().collect()
        });

    let all_ingredients: HashSet<_> = foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .cloned()
        .collect();

    let allergen_free_ingredients: HashSet<_> = all_ingredients
        .difference(&allergen_containing_ingredients)
        .collect();

    allergen_free_ingredients
        .iter()
        .map(|&ingr| {
            foods
                .iter()
                .filter(|f| f.ingredients.contains(ingr))
                .count()
        })
        .sum()
}

fn part2(foods: &[FoodItem]) -> Result<String> {
    let all_allergens: HashSet<_> = foods.iter().flat_map(|f| f.allergens.iter()).collect();
    let mut result: HashMap<String, String> = HashMap::new();
    let mut allergens_to_ingredients: HashMap<String, HashSet<String>> = all_allergens
        .iter()
        .map(|&aller| {
            (
                aller.clone(),
                which_ingredients_might_contain_allergen(aller, foods),
            )
        })
        .collect();

    while result.len() != all_allergens.len() {
        let known = allergens_to_ingredients.iter().find(|(_, v)| v.len() == 1);
        match known {
            Some((aller, ingrs)) => {
                let ingr = ingrs.iter().next().unwrap();
                result.insert(aller.clone(), ingr.clone());
                allergens_to_ingredients = allergens_to_ingredients
                    .iter()
                    .filter(|(k, _)| k != &aller)
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            v.iter().filter(|i| i != &ingr).cloned().collect(),
                        )
                    })
                    .collect();
            }
            None => bail!("Something went way wrong"),
        }
    }

    Ok(result
        .iter()
        .sorted_by_key(|(k, _)| *k)
        .map(|(_, v)| v)
        .join(","))
}

fn main() -> Result<()> {
    let foods = read_lines("input/day21.txt")?;
    let result = part1(&foods);
    println!("part 1: {}", result);

    let result = part2(&foods)?;
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let ingredient_info = read_lines("input/test/day21.txt")?;
        let result = part1(&ingredient_info);
        assert_eq!(result, 5);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let ingredient_info = read_lines("input/test/day21.txt")?;
        let result = part2(&ingredient_info)?;
        assert_eq!(result, "mxmxvkd,sqjhc,fvjkl");
        Ok(())
    }
}
