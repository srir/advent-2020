use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>
}

fn validate_height(hgt: String) -> bool {
    let height = hgt.clone();
    let in_re = Regex::new(r"^(\d+)in$").unwrap();
    let cm_re = Regex::new(r"^(\d+)cm$").unwrap();

    let valid_in = {
        if let Some(_) = in_re.find(height.as_str()) {
            let height_match = in_re.captures_iter(height.as_str()).next().unwrap().get(1).unwrap().as_str();
            let height_in: usize = height_match.parse().unwrap();

            height_in >= 59 && height_in <= 76
        } else {
            false
        }
    };

    let valid_cm = {
        if let Some(_) = cm_re.find(height.as_str()) {
            let height_match = cm_re.captures_iter(height.as_str()).next().unwrap().get(1).unwrap().as_str();
            let height_cm: usize = height_match.parse().unwrap();

            height_cm >= 150 && height_cm <= 193
        } else {
            false
        }
    };

    valid_in || valid_cm
}

fn validate_hair_color(hcl: String) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

    re.is_match(hcl.as_str())
}

fn validate_eye_color(ecl: String) -> bool {
    let valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    valid.contains(&ecl.as_str())
}

fn validate_pid(pid: String) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();

    re.is_match(pid.as_str())
}

fn validate(passport: Passport) -> bool {
    let valid_birth_year  =
        passport.byr.parse().map(|d:usize| d >= 1920 && d <= 2002).unwrap_or(false);

    let valid_issue_year  =
        passport.iyr.parse().map(|d:usize| d >= 2010 && d <= 2020).unwrap_or(false);

    let valid_exp_year  =
        passport.eyr.parse().map(|d:usize| d >= 2020 && d <= 2030).unwrap_or(false);

    let valid_height = validate_height(passport.hgt);

    let valid_hair_color = validate_hair_color(passport.hcl);
    let valid_eye_color = validate_eye_color(passport.ecl);
    let valid_passport_id = validate_pid(passport.pid);


    valid_birth_year && valid_issue_year && valid_exp_year && valid_height
        && valid_hair_color && valid_eye_color && valid_passport_id
}

fn parse_tokens_to_data(tokens: Vec<&str>) -> HashMap<String, String> {
    tokens.iter().filter_map(|tok| {
        let key_val: Vec<&str> = tok.split(":").collect();

        let key = key_val.get(0);
        let val = key_val.get(1);

        if let (Some(key), Some(val)) = (key, val) {
            Some((key.to_string(), val.to_string()))
        } else {
            None
        }
    }).collect::<HashMap<String, String>>()
}

fn parse_passport(tokens: Vec<&str>) -> Option<Passport> {
    let data = parse_tokens_to_data(tokens);

    let all_required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let all_required_present = all_required.iter().all(|key| data.contains_key(&*key.to_string()));

    if all_required_present {
        Some(Passport{
            byr: data.get("byr").unwrap().to_string(),
            iyr: data.get("iyr").unwrap().to_string(),
            eyr: data.get("eyr").unwrap().to_string(),
            hgt: data.get("hgt").unwrap().to_string(),
            hcl: data.get("hcl").unwrap().to_string(),
            ecl: data.get("ecl").unwrap().to_string(),
            pid: data.get("pid").unwrap().to_string(),
            cid: data.get("cid").map(ToString::to_string)
        })
    } else {
        None
    }
}

#[aoc_generator(day4)]
fn parse_passports(data: &str) -> Vec<Option<Passport>> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut passports: Vec<Option<Passport>> = Vec::new();

    data.lines().for_each(|line| {
        if line == "" {
            let passport = parse_passport(tokens.clone());
            passports.push(passport);
            tokens.clear();
        }

        tokens.extend(line.split(" "));
    });

    let passport = parse_passport(tokens.clone());
    passports.push(passport);

    passports
}

#[aoc(day4, part1)]
fn count_valid_passports(passports: &Vec<Option<Passport>>) -> usize {
    passports.iter().filter(|p| p.is_some()).count()
}

#[aoc(day4, part2)]
fn count_valid_passports_2(passports: &Vec<Option<Passport>>) -> usize {
    passports.iter().filter(|p| {
        p.as_ref().map(|v| validate(v.clone())).unwrap_or(false)
    }).count()
}
