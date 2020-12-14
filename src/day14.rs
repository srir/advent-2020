use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum MaskValue {
    Zero,
    One
}

type Mask = Vec<Option<MaskValue>>;

fn parse_mask(s: &str) -> Mask {
    s.chars().map(|char| {
        match char {
            '0' => Some(MaskValue::Zero),
            '1' => Some(MaskValue::One),
            'X' => None,
            _ => unreachable!("couldn't parse mask")
        }
    }).collect()
}

#[derive(Debug, Clone)]
enum Command {
    SetMask(Mask),
    SetMemory { dest: usize, value: usize }
}

#[derive(Debug, Copy, Clone)]
struct CommandParseError;

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        let mask_re = Regex::new(r"mask = (?P<mask>[1|0|X]{36})").unwrap();
        let set_mem_re = Regex::new(r"mem\[(?P<dest>\d+)\] = (?P<val>\d+)").unwrap();

        let maybe_mask = mask_re.captures(s).and_then(|cap| {
            cap.name("mask").map(|mask| parse_mask(mask.as_str()))
        });

        maybe_mask.map(|mask| Command::SetMask(mask)).or_else(|| {
            set_mem_re.captures(s).and_then(|cap| {
                let dest: usize = cap.name("dest").map(|dest| dest.as_str().parse().unwrap()).unwrap();
                let value: usize = cap.name("val").map(|val| val.as_str().parse().unwrap()).unwrap();

                Some(Command::SetMemory { dest, value })
            })
        }).ok_or(CommandParseError)
    }
}

#[aoc_generator(day14)]
fn parse_commands(input: &str) -> Vec<Command> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn masked_value(mask: &Mask, value: &usize) -> usize {
    let mut output: usize = 0;

    for (i, mask_value) in mask.iter().rev().enumerate() {
        match (mask_value, (value >> i) & 1)  {
            (Some(MaskValue::One), _) => {
                output += 1 << i;
            }
            (Some(MaskValue::Zero), _) => {}
            (None, bit_val) => {
                output += bit_val << i;
            }
        }
    };

    output
}

fn sum_memory(memory: &HashMap<usize, usize>) -> usize {
    memory.values().sum()
}

#[aoc(day14, part1)]
fn sum_initialized_memory(commands: &Vec<Command>) -> usize {
    let mut mask: Option<Mask> = None;
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for command in commands {
        match command {
            Command::SetMask(new_mask) => { mask = Some(new_mask.to_vec()) },
            Command::SetMemory {dest, value} => {
                memory.insert(*dest, masked_value(mask.as_ref().unwrap(), value));
            }
        }
    };

    sum_memory(&memory)
}

enum FloatingAddressBit {
    Zero,
    One,
    Floating
}

// reversed
type FloatingAddress = Vec<FloatingAddressBit>;

fn address_mask_to_floating(mask: &Mask, address: &usize) -> FloatingAddress {
    mask.iter().rev().enumerate().map(|(i, mask_value)| {
        match (mask_value, (address >> i) & 1) {
            (None, _) => FloatingAddressBit::Floating,
            (Some(MaskValue::One), _) => FloatingAddressBit::One,
            (Some(MaskValue::Zero), 0) => FloatingAddressBit::Zero,
            (Some(MaskValue::Zero), 1) => FloatingAddressBit::One,
            _ => unreachable!("unexpected bit")
        }
    }).collect()
}

fn all_addresses(floating_addr: &[FloatingAddressBit]) -> Vec<usize> {
    match floating_addr {
        [bit] => {
            match bit {
                FloatingAddressBit::Floating => vec![0, 1],
                FloatingAddressBit::Zero => vec![0],
                FloatingAddressBit::One => vec![1]
            }
        },
        [bit, rest @ ..] => {
            match bit {
                FloatingAddressBit::Floating => {
                    let all = all_addresses(rest);

                    all.iter().map(|addr| {
                        let shifted = addr << 1;
                        vec![shifted, shifted + 1]
                    }).flatten().collect()
                },
                FloatingAddressBit::Zero => {
                    let all = all_addresses(rest);

                    all.iter().map(|addr| addr << 1).collect()
                },
                FloatingAddressBit::One => {
                    let all = all_addresses(rest);

                    all.iter().map(|addr| (addr << 1) + 1).collect()
                }
            }
        },
        _ => unreachable!("cannot compute address")
    }
}

fn mask_to_all_addresses(mask: &Mask, address: &usize) -> Vec<usize> {
    let floating_addr = address_mask_to_floating(mask, address);

    all_addresses(&floating_addr[..])
}

#[aoc(day14, part2)]
fn sum_initialized_memory_2(commands: &Vec<Command>) -> usize {
    let mut mask: Option<Mask> = None;
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for command in commands {
        match command {
            Command::SetMask(new_mask) => { mask = Some(new_mask.to_vec()) },
            Command::SetMemory {dest, value} => {
                for dest in mask_to_all_addresses(mask.as_ref().unwrap(), dest) {
                    memory.insert(dest, *value);
                };
            }
        }
    };

    sum_memory(&memory)
}
