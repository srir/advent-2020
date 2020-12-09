use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn split_to_lines(str: &str) -> Vec<String> {
    str.lines().map(|line| line.to_string()).collect()
}

#[derive(Debug, Copy, Clone)]
struct Slope {
    down: usize,
    right: usize
}

fn count_trees_with_slope(lines: &Vec<String>, slope: Slope) -> u64 {
    let cols = lines.iter().next().unwrap().chars().count();

    let mut col = 0usize;
    let mut count = 0;

    lines.iter().step_by(slope.down).for_each(|line| {
        match line.get(col..col+1) {
            Some("#") => {
                count += 1;
                col = (col + slope.right) % cols;
            }
            Some(_) => {
                col = (col + slope.right) % cols;
            }
            None => ()
        }
    });

    count
}

#[aoc(day3, part1)]
fn count_trees_pt1(lines: &Vec<String>) -> u64 {
    count_trees_with_slope(lines, Slope{ right: 3, down: 1 })
}

#[aoc(day3, part2)]
fn count_trees_pt2(lines: &Vec<String>) -> u64 {
    let slopes = [
        Slope{ right: 1, down: 1},
        Slope{ right: 3, down: 1},
        Slope{ right: 5, down: 1},
        Slope{ right: 7, down: 1},
        Slope{ right: 1, down: 2},
    ];

    let counts: Vec<u64> = slopes.iter()
        .map(|&slope| count_trees_with_slope(lines, slope))
        .collect();

    counts.iter().fold(1, |a, b| a * b)
}
