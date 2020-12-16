use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::collections::{HashMap, HashSet};

#[derive(Debug, FromStr, Display, Copy, Clone, Eq, PartialEq, Hash)]
#[display("{min}-{max}")]
struct Range {
    min: usize,
    max: usize
}

impl Range {
    fn is_within(&self, value: usize) -> bool {
        self.min <= value && value <= self.max
    }
}

type Ticket = Vec<usize>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Rule {
    name: String,
    ranges: Vec<Range>
}

impl Rule {
    fn valid_value(&self, value: usize) -> bool {
        self.ranges.iter().any(|range| {
            range.is_within(value)
        })
    }
}

#[derive(Debug, Clone)]
struct Data {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>
}

impl Data {
    fn invalid_values_for_ticket(&self, ticket: &Ticket) -> Vec<usize> {
        ticket.iter().filter(|&&value| {
            self.rules.iter().all(|rule| !rule.valid_value(value))
        }).cloned().collect()
    }

    fn invalid_values_for_nearby_tickets(&self) -> Vec<Vec<usize>> {
        self.nearby_tickets.iter().map(|ticket| self.invalid_values_for_ticket(ticket)).collect()
    }

    fn ticket_scanning_error_rate(&self) -> usize {
        self.invalid_values_for_nearby_tickets().iter().flatten().sum()
    }

    fn valid_nearby_tickets(&self) -> Vec<Ticket> {
        self.nearby_tickets.iter().filter(|&ticket| {
            self.invalid_values_for_ticket(ticket).len() == 0
        }).cloned().collect()
    }

    fn assign_field_names(&self) -> HashMap<Rule, usize> {
        let tickets = self.valid_nearby_tickets();
        let num_fields = tickets.first().unwrap().len();

        let mut candidates: HashMap<usize, HashSet<Rule>> = (0..num_fields).map(|field| {
            (field, self.rules.iter().filter(|rule| {
                tickets.iter().all(|ticket| {
                    rule.valid_value(ticket[field])
                })
            }).cloned().collect())
        }).collect();

        let mut assignments = HashMap::new();

        while !candidates.is_empty() {
            let (&field, ruleset) = candidates.iter().filter(|&(_, ruleset)| {
                ruleset.len() == 1
            }).next().unwrap();

            let rule = ruleset.iter().next().unwrap().clone();
            assignments.insert(rule.clone(), field);

            // delete field from candidates, and delete rule from everything
            candidates.values_mut().for_each(|ruleset| {
                ruleset.remove(&rule);
            });
            candidates.remove(&field);
        }

        assignments
    }

}

fn parse_rules(section: &str) -> Vec<Rule> {
    section.lines().map(|line| {
        let i = line.find(":").unwrap();

        let name = &line[0..i];
        let ranges_text = &line[i+2..];

        let ranges = ranges_text
            .split(" or ")
            .map(|range_text| range_text.parse::<Range>().unwrap()).collect();

        Rule {
            name: name.to_string(),
            ranges
        }
    }).collect()
}

fn parse_ticket(line: &str) -> Ticket {
    line.split(",").map(|i| i.parse().unwrap()).collect()
}

fn parse_my_ticket(section: &str) -> Ticket {
    let data: Vec<&str> = section.lines().collect();

    parse_ticket(data[1])
}

fn parse_nearby_tickets(section: &str) -> Vec<Ticket> {
    section.lines().skip(1).map(|l| parse_ticket(l)).collect()
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Data {
    let sections: Vec<&str> = input.split("\n\n").collect();

    Data {
        rules: parse_rules(sections[0]),
        my_ticket: parse_my_ticket(sections[1]),
        nearby_tickets: parse_nearby_tickets(sections[2])
    }
}

#[aoc(day16, part1)]
fn ticket_scanning_error_rate(data: &Data) -> usize {
    data.ticket_scanning_error_rate()
}

#[aoc(day16, part2)]
fn departure_fields_product(data: &Data) -> usize {
    let assignments = data.assign_field_names();

    let fields = assignments.keys().filter(|rule| {
        rule.name.starts_with("departure")
    }).map(|r| assignments.get(r).unwrap());

    fields.map(|&f| data.my_ticket[f]).product()
}
