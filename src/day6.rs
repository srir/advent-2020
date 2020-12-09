use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::iter::FromIterator;

type Answers = HashSet<char>;

#[derive(Debug, Clone)]
struct Group {
    answers: Vec<Answers>
}

impl Group {
    fn count_any_yeses(&self) -> usize {
        let merged = self.answers
            .iter()
            .fold(HashSet::new() as Answers, |acc, answers| {
                acc.union(&answers).cloned().collect()
            });

        merged.len()
    }

    fn count_all_yeses(&self) -> usize {
        let merged = self.answers
            .iter()
            .skip(1)
            .fold(self.answers[0].clone(), |acc, answers| {
                acc.intersection(answers).cloned().collect()
            });

        merged.len()
    }
}

#[aoc_generator(day6)]
fn parse_groups_answers(input: &str) -> Vec<Group> {
    let mut output : Vec<Group> = vec![];

    let mut group_answers : Vec<Answers> = vec![];

    input.lines().for_each(|line| {
        if line == "" {
            output.push(Group{ answers: group_answers.clone() });
            group_answers.clear();
        } else {
            let indiv_answers = HashSet::from_iter(line.chars());
            group_answers.push(indiv_answers);
        }
    });

    output.push(Group{ answers: group_answers.clone() });

    output
}

#[aoc(day6, part1)]
fn sum_group_any_yeses(groups: &Vec<Group>) -> usize {
    groups.iter().map(|g| g.count_any_yeses()).sum()
}

#[aoc(day6, part2)]
fn sum_group_all_yeses(groups: &Vec<Group>) -> usize {
    groups.iter().map(|g| g.count_all_yeses()).sum()
}
