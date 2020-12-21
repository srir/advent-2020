use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

type Ingredient = String;
type Allergen = String;

#[derive(Debug, Clone)]
struct Food {
    ingredients: Vec<Ingredient>,
    allergens: Vec<Allergen>
}

#[derive(Debug, Clone)]
struct Data {
    foods: Vec<Food>
}

impl Data {
    fn all_allergens(&self) -> HashSet<Allergen> {
        self.foods.iter().map(|food| food.allergens.clone()).flatten().collect()
    }

    fn potential_ingredients_for_allergen(&self, allergen: &Allergen) -> HashSet<Ingredient> {
        let foods_with_allergen = self.foods.iter().filter(|&food| {
            food.allergens.contains(&allergen)
        });

        let ingredient_lists = foods_with_allergen
            .map(|food| food.ingredients.iter().cloned().collect::<HashSet<Ingredient>>());

        ingredient_lists.fold1(|acc, ings| {
            acc.intersection(&ings).cloned().collect()
        }).unwrap_or(HashSet::new())
    }

    fn potential_allergenic_ingredients(&self) -> HashSet<Ingredient> {
        let all_allergens = self.all_allergens();

        all_allergens.iter().map(|allergen| {
            self.potential_ingredients_for_allergen(allergen)
        }).fold(HashSet::new(), |acc, ings| {
            acc.union(&ings).cloned().collect()
        })
    }

    fn nonallergenic_ingredients(&self) -> HashSet<Ingredient> {
        let all_ingredients = self.foods.iter()
            .map(|food| food.ingredients.iter().cloned().collect::<HashSet<Ingredient>>())
            .fold(HashSet::new(), |acc, ings| {
                acc.union(&ings).cloned().collect()
            });

        let all_candidates = self.potential_allergenic_ingredients();

        all_ingredients.difference(&all_candidates).cloned().collect()
    }

    fn count_occurrences(&self, ingredient: &Ingredient) -> usize {
        self.foods.iter().map(|food| {
            food.ingredients.iter().filter(|&i| i == ingredient).count()
        }).sum()
    }

    fn count_all_occurrences(&self, ingredients: &Vec<Ingredient>) -> usize {
        ingredients.iter().map(|ing| self.count_occurrences(ing)).sum()
    }
}

fn parse_line(input: &str) -> Food {
    let _contains = "(contains ";
    let _contains_start = input.find(_contains).expect("invalid ingredient list");

    let ingredients_str = &input[0.._contains_start];
    let allergens_str = &input[_contains_start + _contains.len() .. input.len() - 1];

    Food {
        ingredients: ingredients_str.split_whitespace().map( | s| s.to_string()).collect(),
        allergens: allergens_str.split(", ").map(|s| s.to_string()).collect()
    }
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Data {
    Data {
        foods: input.lines().map(|line| { parse_line(line) }).collect()
    }
}

#[aoc(day21, part1)]
fn count_nonallergenic(foods: &Data) -> usize {
    let ings = foods.nonallergenic_ingredients();

    foods.count_all_occurrences(&ings.iter().cloned().collect())
}

fn _find_next_minimal(candidates: &HashMap<Allergen, HashSet<Ingredient>>) -> Option<(Allergen, Ingredient)> {
    candidates.iter().find(|&(_, v)| {
        v.iter().len() == 1
    }).map(|(a, is)| (a.clone(), is.iter().next().unwrap().clone()))
}

#[aoc(day21, part2)]
fn canonical_dangerous_ingredient_list(foods: &Data) -> String {
    let allergens = foods.all_allergens();
    let mut allergen_ingredients = HashMap::<Allergen, Ingredient>::new();

    let mut allergen_candidates_map = allergens.iter().map(|allergen| {
        (allergen.clone(), foods.potential_ingredients_for_allergen(allergen))
    }).collect::<HashMap<Allergen, HashSet<Ingredient>>>();

    while let Some((a, i)) = _find_next_minimal(&allergen_candidates_map) {
        allergen_ingredients.insert(a.clone(), i.clone());
        allergen_candidates_map.remove(a.as_str());

        for (_, candidates) in allergen_candidates_map.iter_mut() {
            candidates.remove(i.as_str());
        }
    }

    let ingredients = allergen_ingredients.iter()
        .sorted_by_key(|&(a, _)| a)
        .map(|(_, i)| i)
        .cloned()
        .collect::<Vec<Ingredient>>();

    ingredients.join(",")
}
