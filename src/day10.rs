use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day10)]
fn parse_joltages(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).sorted().collect()
}

#[aoc(day10, part1)]
fn jolt_diff_product(input: &Vec<usize>) -> usize {
    let mut last = 0usize;
    let mut counts: HashMap<usize, usize> = HashMap::new();

    input.iter().for_each(|&v| {
        assert!(v - last > 0 && v - last <= 3);

        *counts.entry(v - last).or_insert(0) += 1;
        last = v;
    });

    counts.get(&1usize).unwrap() * (counts.get(&3usize).unwrap() + 1)
}

#[aoc(day10, part2)]
fn jolt_count_arrangements(input: &Vec<usize>) -> usize {
    let mut cache = HashMap::new();

    input.iter().take(3).map(|&v| {
        if v <= 3 {
            _count_arrangements_cached(input, &mut cache)
        } else {
            0
        }
    }).sum()
}

fn _count_arrangements_cached(input: &[usize], cache: &mut HashMap<usize, usize>) -> usize {
    let val = match &input[..] {
        [] => 1,
        [_] => 1,
        [current, rest @ ..] => {
            // first check the cache!
            if let Some(v) = cache.get(current) {
                *v
            } else {
                // can check up to 3 next items
                let next_3 = rest.iter().take(3);

                let sum_vals = next_3.enumerate().map(|(index, v)| {
                    if v - current <= 3 {
                        _count_arrangements_cached(&rest[index..], cache)
                    } else {
                        0
                    }
                }).sum();

                cache.insert(*current, sum_vals);

                sum_vals
            }
        }
    };

    val
}
