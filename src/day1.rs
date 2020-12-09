use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn split_to_numbers(str: &str) -> Vec<u32> {
    str.lines().map(|line| line.parse().unwrap()).collect()
}

fn mult_matches(matches: Vec<Vec<u32>>) -> u128 {
    matches
        .iter()
        .map(|match_vals| match_vals.iter().fold(1u128, |a, &b| a * u128::from(b)))
        .next()
        .unwrap()
}

#[aoc(day1, part1)]
fn two_elem_sum(nums: &Vec<u32>) -> u128 {
    let mut results = Vec::new();

    for (i, &num1) in nums.iter().enumerate() {
        for &num2 in nums.iter().skip(i + 1) {
            if num1 + num2 == 2020 {
                results.push(vec![num1, num2])
            }
        }
    }

    mult_matches(results)
}

#[aoc(day1, part2)]
fn three_elem_sum(nums: &Vec<u32>) -> u128 {
    let mut results = Vec::new();

    for (i, &num1) in nums.iter().enumerate() {
        for (j, &num2) in nums.iter().skip(i + 1).enumerate() {
            for (_, &num3) in nums.iter().skip(j + 1).enumerate() {
                if num1 + num2 + num3 == 2020 {
                    results.push(vec![num1, num2, num3]);
                }
            }
        }
    }

    mult_matches(results)
}

