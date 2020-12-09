use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_to_numbers(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn is_sum_of_two(preamble: &Vec<usize>, sum: usize) -> bool {
    preamble.iter().enumerate().any(|(i, val1)| {
        preamble.iter().skip(i).enumerate().any(|(_, val2)| {
            val1 + val2 == sum
        })
    })
}

#[aoc(day9, part1)]
fn first_not_matching(inputs: &Vec<usize>) -> usize {
    let mut preamble: Vec<usize> = inputs.iter().take(25).cloned().collect();

    for &elem in inputs.iter().skip(25) {
        if !is_sum_of_two(&preamble, elem) {
            return elem;
        } else {
            preamble.drain(0..1);
            preamble.push(elem);
        }
    }

    0
}

fn contiguous_subsequence_sum(inputs: &Vec<usize>, target: usize) -> Option<&[usize]> {
    for (i, _) in inputs.iter().enumerate() {
        for (j, _) in inputs.iter().enumerate() {
            if j < i+1 {
                continue;
            }

            let subseq = inputs.get(i..j).unwrap();

            if subseq.iter().sum::<usize>() == target {
                return Some(subseq)
            }
        }
    };

    None
}

#[aoc(day9, part2)]
fn encryption_weakness(inputs: &Vec<usize>) -> usize {
    let target = 257342611;

    let subseq = contiguous_subsequence_sum(inputs, target).unwrap();

    let max = subseq.iter().max().unwrap();
    let min = subseq.iter().min().unwrap();

    min + max
}
