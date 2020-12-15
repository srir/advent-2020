use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<usize> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}

fn value_at_iteration(input: &Vec<usize>, iteration: usize) -> usize {
    let mut previous_turns: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut last_value: Option<usize> = None;

    for i in 1..=iteration {
        last_value = if let Some(&v) = input.get(i - 1) {
            Some(v)
        } else {
            let prev_turns =
                previous_turns.get(&last_value.unwrap()).map(|res| &res[..]);
            Some(match prev_turns {
                None | Some([_]) => 0,
                Some([.., prev_turn, last_turn]) => last_turn - prev_turn,
                _ => unreachable!("nope")
            })
        };

        previous_turns.entry(last_value.unwrap()).or_insert(vec![]).push(i);
    }

    last_value.unwrap()
}

#[aoc(day15, part1)]
fn iteration_2020_number(input: &Vec<usize>) -> usize {
    value_at_iteration(input, 2020)
}

#[aoc(day15, part2)]
fn iteration_30000000_number(input: &Vec<usize>) -> usize {
    value_at_iteration(input, 30000000)
}
