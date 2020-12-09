use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Color = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BagType {
    modifier: String,
    color: Color
}

type Ruleset = HashMap<BagType, Vec<(usize, BagType)>>;

fn parse_bag_type_with_count(input: &str) -> (usize, BagType) {
    let re = Regex::new(r"(\d+) (\w+) (\w+) (?:bag|bags)").unwrap();

    let captures = re.captures(input).unwrap();

    let count = captures.get(1).unwrap().as_str().parse().unwrap();
    let modifier = captures.get(2).unwrap().as_str().to_string();
    let color = captures.get(3).unwrap().as_str().to_string();

    (count, BagType{modifier, color})
}

#[aoc_generator(day7)]
fn parse_rules(input: &str) -> Ruleset {
    let re = Regex::new(r"^(\w+) (\w+) bags contain (.*).$").unwrap();

    input.lines().map(|line| {
        let captures = re.captures(line).unwrap();

        let modifier = captures.get(1).unwrap().as_str().to_string();
        let color = captures.get(2).unwrap().as_str().to_string();
        let rest = captures.get(3).unwrap().as_str();

        let bag_type = BagType{modifier, color};

        if rest == "no other bags" {
            (bag_type, vec![])
        } else {
            let bag_type_strs = rest.split(", ");

            (bag_type, bag_type_strs.map(|s| parse_bag_type_with_count(s)).collect())
        }
    }).collect()
}

fn reverse_lookup(rules: &Ruleset, target: &BagType) -> Vec<BagType> {
    let mut containers = vec![];

    rules.iter().for_each(|(bag_type, contained)| {
        if contained.iter().any(|(_, b)| b == target) {
            containers.push(bag_type.clone());
        }
    });

    containers
}


#[aoc(day7, part1)]
fn count_rules_with_bag(rules: &Ruleset) -> usize {
    let mut to_lookup: HashSet<BagType> = HashSet::new();
    let mut found: HashSet<BagType> = HashSet::new();

    to_lookup.insert(BagType { modifier: String::from("shiny"), color: String::from("gold") });

    while !to_lookup.is_empty() {
        let frontier = to_lookup.clone();
        to_lookup.clear();

        for bag_type in frontier {
            for found_type in reverse_lookup(rules, &bag_type).iter() {
                if !found.contains(found_type) {
                    found.insert(found_type.clone());
                    to_lookup.insert(found_type.clone());
                }
            }
        }
    }

    found.len()
}

fn count_bags_for_target(
    rules: &Ruleset,
    cache: &mut HashMap<BagType, usize>,
    target: &BagType
) -> usize {
    if let Some(v) = cache.get(target) {
        return *v;
    }


    let rule = rules.get(target).unwrap();
    let mut sum = 0;

    for (bag_count, bag_type) in rule.iter() {
        sum += bag_count * (1 + count_bags_for_target(rules, cache, bag_type));
    }

    cache.insert(target.clone(), sum);

    sum
}

#[aoc(day7, part2)]
fn count_bags(rules: &Ruleset) -> usize {
    let mut cache: HashMap<BagType, usize> = HashMap::new();

    let target = BagType { modifier: String::from("shiny"), color: String::from("gold") };

    count_bags_for_target(rules, &mut cache, &target)
}

