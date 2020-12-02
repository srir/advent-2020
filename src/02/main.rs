use std::{io, io::prelude::*};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct PasswordPolicy {
    letter: char,
    min_times: u32,
    max_times: u32,
}

impl FromStr for PasswordPolicy {
    type Err = io::Error;

    fn from_str(policy_str: &str) -> Result<Self, Self::Err> {
        let policy_tokens: Vec<&str> = policy_str.split(" ").collect();
        let (range, letter) = (policy_tokens[0], policy_tokens[1]);

        let range_tokens: Vec<&str> = range.split("-").collect();
        let (min, max) = (range_tokens[0], range_tokens[1]);

        Ok(PasswordPolicy{
            letter: letter.chars().next().unwrap(),
            min_times: min.parse().unwrap(),
            max_times: max.parse().unwrap()
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct PasswordPolicy2 {
    letter: char,
    first_pos: usize,
    second_pos: usize,
}

impl FromStr for PasswordPolicy2 {
    type Err = io::Error;

    fn from_str(policy_str: &str) -> Result<Self, Self::Err> {
        let policy_tokens: Vec<&str> = policy_str.split(" ").collect();
        let (range, letter) = (policy_tokens[0], policy_tokens[1]);

        let range_tokens: Vec<&str> = range.split("-").collect();
        let (first, second) = (range_tokens[0], range_tokens[1]);

        Ok(PasswordPolicy2{
            letter: letter.chars().next().unwrap(),
            first_pos: first.parse().unwrap(),
            second_pos: second.parse().unwrap()
        })
    }
}

fn parse_line(line: String) -> (PasswordPolicy, String) {
    let tokens: Vec<&str> = line.split(": ").collect();

    (PasswordPolicy::from_str(tokens[0]).unwrap(), tokens[1].to_string())
}

fn parse_line_2(line: String) -> (PasswordPolicy2, String) {
    let tokens: Vec<&str> = line.split(": ").collect();

    (PasswordPolicy2::from_str(tokens[0]).unwrap(), tokens[1].to_string())
}

fn validate_password(policy: PasswordPolicy, password: String) -> bool {
    let count = password.chars().fold(0, |acc, char| {
        if char == policy.letter { acc + 1 } else { acc }
    });

    count >= policy.min_times && count <= policy.max_times
}

fn validate_password_2(policy: PasswordPolicy2, password: String) -> bool {
    let password_bytes = password.as_bytes();

    (password_bytes[policy.first_pos - 1] as char == policy.letter)
        ^ (password_bytes[policy.second_pos - 1] as char  == policy.letter)
}

fn count_valid_passwords(policy_password_pairs: Vec<(PasswordPolicy, String)>) -> u64 {
    policy_password_pairs.iter()
        .filter(|(policy, password)| validate_password(policy.clone(), password.to_string()))
        .count() as u64
}

fn count_valid_passwords_2(policy_password_pairs: Vec<(PasswordPolicy2, String)>) -> u64 {
    policy_password_pairs.iter()
        .filter(|(policy, password)| validate_password_2(policy.clone(), password.to_string()))
        .count() as u64
}

fn main() -> io::Result<()> {
    let policy_password_pairs_2: Vec<(PasswordPolicy2, String)> = io::stdin().lock().lines().filter_map(|line| {
        match line {
            Err(_) => None,
            Ok(line) => Some(parse_line_2(line))
        }
    }).collect();

    let valid_count = count_valid_passwords_2(policy_password_pairs_2);

    println!("{}", valid_count);

    Ok(())
}
