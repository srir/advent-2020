use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use regex::Regex;

type RuleId = usize;

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Subrules(Vec<Vec<RuleId>>)
}

fn parse_subrules(input: &str) -> Option<Rule> {
    let subrules = input.split("|").map(|subrule| {
        subrule.trim().split_whitespace().map(|r| r.parse()).collect::<Result<Vec<RuleId>, _>>().ok()
    }).collect::<Option<Vec<Vec<RuleId>>>>();

    subrules.map(|sr| Rule::Subrules(sr))
}

fn parse_char(input: &str) -> Option<Rule> {
    let re = Regex::new(r#""(?P<char>\w)""#).unwrap();

    re.captures(input).and_then(|cap| {
        cap.name("char").map(|c| {
            let char = c.as_str().chars().next().unwrap();

            Rule::Char(char)
        })
    })
}

fn parse_rule(rule_str: &str) -> Rule {
    parse_char(rule_str)
        .or_else(|| parse_subrules(rule_str))
        .expect("couldn't parse rule")
}

fn parse_rule_with_key(line: &str) -> (RuleId, Rule) {
    let key_split = line.split(": ").collect::<Vec<&str>>();
    let key = key_split[0].parse::<RuleId>().unwrap();

    let rule_str = key_split[1];

    (key, parse_rule(rule_str))
}

type Ruleset = HashMap<RuleId, Rule>;

fn parse_ruleset(input: &str) -> Ruleset {
    let mut context = HashMap::new();

    for line in input.lines() {
        let (key, rule) = parse_rule_with_key(line);

        context.insert(key, rule);
    }

    context
}

fn eval_rules(context: &Ruleset, rs: &[RuleId], input: &str) -> bool {
    match rs {
        [] => input.len() == 0,
        [rule_id, rest @ ..] => {
            match context.get(rule_id).expect("couldn't find rule") {
                Rule::Char(c) => input.starts_with(c.to_string().as_str())
                    && eval_rules(context, rest, &input[1..]),
                Rule::Subrules(subrules) => {
                    subrules.iter().any(|subrule| {
                        let expanded =
                            subrule.iter().chain(rest.iter()).cloned().collect::<Vec<RuleId>>();

                        eval_rules(context, &*expanded, input)
                    })
                }
            }
        }
    }
}

struct Data {
    rules: Ruleset,
    messages: Vec<String>
}

impl Data {
    fn count_messages_matching_rule(&self, rule_id: usize) -> usize {
        let rule_ids =vec![rule_id];

        self.messages.iter()
            .filter(|&msg| {
                eval_rules(&self.rules, &rule_ids, msg.as_str())
            })
            .count()
    }
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Data {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let rules = parse_ruleset(sections[0]);

    let messages = sections[1].lines()
        .map(|s| s.to_string())
        .collect();

    Data {
        rules,
        messages
    }
}

#[aoc(day19, part1)]
fn part1(data: &Data) -> usize {
    data.count_messages_matching_rule(0)
}

#[aoc(day19, part2)]
fn part2(data: &Data) -> usize {
    let mut new_rules = data.rules.clone();

    new_rules.insert(8, Rule::Subrules(vec![vec![42], vec![42, 8]]));
    new_rules.insert(11, Rule::Subrules(vec![vec![42, 31], vec![42, 11, 31]]));

    let data = Data {
        rules: new_rules,
        messages: data.messages.clone()
    };

    data.count_messages_matching_rule(0)
}
