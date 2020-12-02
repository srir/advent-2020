use std::{io, io::prelude::*};

fn mult_matches(matches: Vec<Vec<u32>>) -> u128 {
    matches
        .iter()
        .map(|match_vals| match_vals.iter().fold(1u128, |a, &b| a * u128::from(b)))
        .next()
        .unwrap()
}

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

fn main() -> io::Result<()> {
    let nums: Vec<u32> = io::stdin().lock().lines().filter_map(|line| line.ok()?.parse().ok()).collect();

    let two_elem = two_elem_sum(&nums);
    println!("{}", two_elem);

    let three_elem = three_elem_sum(&nums);
    println!("{}", three_elem);

    Ok(())
}
