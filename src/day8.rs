use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Instr {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

type Program = Vec<Instr>;

#[aoc_generator(day8)]
fn parse_program(input: &str) -> Program {
    input.lines().map(|line| {
        match &line[0..3] {
            "nop" => Instr::Nop(line[4..].parse().unwrap()),
            "acc" => Instr::Acc(line[4..].parse().unwrap()),
            "jmp" => Instr::Jmp(line[4..].parse().unwrap()),
            _ => unreachable!("unexpected program instruction")
        }
    }).collect()
}

enum RunResult {
    Terminated(isize),
    Loop(isize)
}

fn run_program_once(program: &Program) -> RunResult {
    let len = program.len();

    let mut acc = 0isize;
    let mut pc = 0usize;

    let mut visited: HashMap<usize, usize> = HashMap::new();

    while visited.get(&pc).unwrap_or(&0) == &0 {
        *visited.entry(pc).or_insert(0) += 1;

        if pc == len {
            return RunResult::Terminated(acc)
        }

        assert!(pc < len);

        let cmd = program[pc];
        match cmd {
            Instr::Nop(_) => {
                pc += 1;
            },
            Instr::Acc(x) => {
                acc += x;
                pc += 1
            }
            Instr::Jmp(j) => {
                pc = (pc as isize + j) as usize;
            }
        }
    }

    RunResult::Loop(acc)
}

#[aoc(day8, part1)]
fn first_iteration_acc(program: &Program) -> isize {
    let rr = run_program_once(program);

    match rr {
        RunResult::Loop(acc) => acc,
        _ => unreachable!("unexpected run result")
    }
}

#[aoc(day8, part2)]
fn terminate_acc(program: &Program) -> isize {
    let mut res: Option<isize> = None;

    program.iter().enumerate().find(|&(idx, &instr)| {
        let new_instr = match instr {
            Instr::Nop(j) => Some(Instr::Jmp(j)),
            Instr::Jmp(j) => Some(Instr::Nop(j)),
            Instr::Acc(_) => None
        };

        if new_instr.is_none() {
            false
        } else {
            let new_instr = new_instr.unwrap();

            let mut new_program = program.clone();
            new_program.splice(idx..idx+1, vec![new_instr]);

            let result = run_program_once(&new_program);

            if let RunResult::Terminated(acc) = result {
                res = Some(acc);

                true
            } else {
                false
            }
        }
    });

    res.unwrap()
}
