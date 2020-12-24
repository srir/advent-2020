use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Cup = usize;

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<Cup> {
    input.chars().map(|char| {
        char.to_digit(10).unwrap() as usize
    }).collect()
}

#[derive(Debug, Clone)]
struct Game {
    min: usize,
    max: usize,
    position: usize,
    cups: Vec<Cup>
}

impl Game {
    fn new(cups: Vec<Cup>, max: usize) -> Game {
        Game {
            position: 0,
            cups,
            min: 0,
            max
        }
    }

    fn rotate_to_zero(&mut self) -> () {
        // println!("before: pos: {}, cups: {:?}", self.position, self.cups);

        self.cups = self.cups.iter().cycle().cloned().skip(self.position).take(self.max).collect();
        self.position = 0;

        // println!("after: pos: {}, cups: {:?}", self.position, self.cups);
    }

    fn cups_after_one(&self) -> Vec<Cup> {
        let (one_pos, _) = self.cups.iter().find_position(|&&c| c == 1).unwrap();

        let num_to_take = self.cups.len() - 1;
        let starting_idx = one_pos + 1;

        self.cups.iter().cycle().cloned().skip(starting_idx).take(num_to_take).collect()
    }

    fn find_dest(&self) -> (usize, &Cup) {
        // println!("find dest");
        let maybe_target = self.cups[0] - 1;
        let first_target = if maybe_target > 0 {
            maybe_target
        } else {
            self.max
        };

        let range_low = (self.min..=first_target).rev();
        let range_high = (first_target+1..=self.max).rev();

        // println!("{:?}, {:?}", range_low, range_high);

        range_low.chain(range_high).find_map(|dest_value| {
            self.cups.iter().skip(4).find_position(|&&x| x == dest_value)
        }).unwrap()
    }

    fn do_move(&mut self) -> () {
        // println!("do move");
        assert_eq!(self.position, 0);

        let (dest_idx, _value) = self.find_dest();

        // println!("dest_idx: {}, value: {}", dest_idx, value);

        let picked_up = self.cups.drain(1..4).collect::<Vec<Cup>>();

        self.cups.splice(dest_idx+2..dest_idx+2, picked_up.iter().cloned());
        self.position += 1;

        // println!("cups: {:?}", self.cups);

        // println!("move mostly done");
        self.rotate_to_zero();
    }
}


#[aoc(day23, part1)]
fn ordering_after_100_cycles(input: &Vec<Cup>) -> String {
    let mut game = Game::new(input.to_vec(), *input.iter().max().unwrap());

    for _ in 0..100 {
        game.do_move();
    }

    game.cups_after_one().iter().join("")
}

#[aoc(day23, part2)]
fn star_cups_product(input: &Vec<Cup>) -> usize {
    let max = *input.iter().max().unwrap();
    let extended_input = input.iter().cloned().chain(max+1..=1000000).collect::<Vec<Cup>>();

    let mut game = Game::new(extended_input, 1000000);

    for i in 0..10000000 {
        if i % 100000 == 0 {
            println!("{}%", i / 100000);
        }

        game.do_move();
    }

    0
}
