use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day25)]
fn parse_input(input: &str) -> (usize, usize) {
    let lines = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<usize>>();

    (lines[0], lines[1])
}

fn transform(subject_number: usize, input: usize) -> usize {
    let res = input * subject_number;
    res % 20201227
}

fn transform_n(subject_number: usize, loop_number: usize) -> usize {
    let mut res = 1;

    for _ in 0..loop_number {
        res = transform(subject_number, res)
    }

    res
}

#[aoc(day25, part1)]
fn encryption_key(&(card_pubkey, door_pubkey): &(usize, usize)) -> usize {
    let subject_number = 7;

    let mut card_loop_count = 0;
    let mut card_target = 1;
    while card_target != card_pubkey {
        card_target = transform(subject_number, card_target);
        card_loop_count += 1;
    }

    transform_n(door_pubkey, card_loop_count)
}
