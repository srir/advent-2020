use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

enum Dir {
    Up,
    Down
}

fn bsearch(min: usize, max: usize, dirs: &Vec<Dir>) -> usize {
    let mut lower = min;
    let mut upper = max;
    let mut mid = (lower + upper) / 2;

    dirs.iter().for_each(|dir| {
        match dir {
            Dir::Up => {
                lower = mid;
            },
            Dir::Down => {
                upper = mid;
            }
        }

        mid = (lower + upper) / 2;
    });

    mid
}

#[derive(Debug, Copy, Clone)]
struct Position {
    row: usize,
    col: usize
}

impl Position {
    fn seat_number(&self) -> usize {
        self.row * 8 + self.col
    }
}

fn code_to_position(code: &str) -> Position {
    let vert = &code[..7];
    let horiz = &code[7..];

    let vert_dirs: Vec<Dir> =
        vert.chars().map(|char| if char == 'F' { Dir::Down } else { Dir::Up }).collect();
    let horiz_dirs: Vec<Dir> =
        horiz.chars().map(|char| if char == 'L' { Dir::Down } else { Dir::Up }).collect();

    Position{
        row: bsearch(0, 128, &vert_dirs),
        col: bsearch(0, 8, &horiz_dirs)
    }
}

#[aoc_generator(day5)]
fn rows_to_codes(input: &str) -> Vec<Position> {
    input.lines().map(|s| code_to_position(s)).collect()
}

#[aoc(day5, part1)]
fn highest_seat_number(positions: &Vec<Position>) -> usize {
    positions.iter().map(|p| p.seat_number()).max().unwrap()
}

fn skipped_seat_numbers(positions: &Vec<Position>) -> Vec<usize> {
    let sorted = positions.iter().map(|p| p.seat_number()).sorted();

    let mut skipped: Vec<usize> = Vec::new();
    let mut last = 0usize;

    sorted.for_each(|num| {
        if last != 0 && last + 1 < num {
            skipped.push(last + 1);
        }

        last = num;
    });

    skipped
}

#[aoc(day5, part2)]
fn my_seat_number(positions: &Vec<Position>) -> usize {
    let skipped = skipped_seat_numbers(positions);

    skipped[0]
}
