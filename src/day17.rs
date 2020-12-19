use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::ops::RangeInclusive;

type Coord3 = (isize, isize, isize);
type Coord4 = (isize, isize, isize, isize);

#[derive(Debug, Copy, Clone)]
enum State {
    Active,
    Inactive
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> HashMap<Coord3, State> {
    let mut data = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let value = match char {
                '.' => State::Inactive,
                '#' => State::Active,
                _ => unreachable!("unrecognized input char")
            };

            data.insert((x as isize, y as isize, 0), value);
        }
    }

    data
}

// count of (Active, Inactive)
fn count_neighboring_states_3(input: &HashMap<Coord3, State>, coord: &Coord3) -> (usize, usize) {
    let &(x0, y0, z0) = coord;

    let mut actives = 0;
    let mut inactives = 0;

    for x in x0-1 ..= x0+1 {
        for y in y0-1 ..= y0+1 {
            for z in z0-1 ..= z0+1 {
                if (x, y, z) != (x0, y0, z0) {
                    match input.get(&(x, y, z)).unwrap_or(&State::Inactive) {
                        State::Inactive => { inactives += 1; }
                        State::Active => { actives += 1; }
                    }
                }
            }
        }
    }

    (actives, inactives)
}

type CoordRange3 = (RangeInclusive<isize>, RangeInclusive<isize>, RangeInclusive<isize>);

fn coord_range_3(data: &HashMap<Coord3, State>) -> CoordRange3 {
    let mut x_min = None;
    let mut y_min = None;
    let mut z_min = None;
    let mut x_max = None;
    let mut y_max = None;
    let mut z_max = None;

    let keys: Vec<&Coord3> = data.keys().collect();

    for &(x, y, z) in keys {
        if x_min.is_none() || x < x_min.unwrap() {
            x_min = Some(x);
        }

        if y_min.is_none() || y < y_min.unwrap() {
            y_min = Some(y);
        }

        if z_min.is_none() || z < z_min.unwrap() {
            z_min = Some(z);
        }

        if x_max.is_none() || x > x_max.unwrap() {
            x_max = Some(x);
        }

        if y_max.is_none() || y > y_max.unwrap() {
            y_max = Some(y);
        }

        if z_max.is_none() || z > z_max.unwrap() {
            z_max = Some(z);
        }
    }

    (
        x_min.unwrap() - 1 ..= x_max.unwrap() + 1,
        y_min.unwrap() - 1 ..= y_max.unwrap() + 1,
        z_min.unwrap() - 1 ..= z_max.unwrap() + 1
    )
}

fn step_one_cycle_3(input: &HashMap<Coord3, State>) -> HashMap<Coord3, State> {
    let mut output = HashMap::new();

    let (x_range, y_range, z_range) = coord_range_3(input);

    for x in x_range {
        for y in y_range.clone() {
            for z in z_range.clone() {
                let coord = (x, y, z);

                let (actives, _inactives) = count_neighboring_states_3(input, &coord);

                let current_state = input.get(&coord).unwrap_or(&State::Inactive);

                match current_state {
                    State::Active => {
                        if actives == 2 || actives == 3 {
                            output.insert(coord, State::Active);
                        } else {
                            output.insert(coord, State::Inactive);
                        }
                    },
                    State::Inactive => {
                        if actives == 3 {
                            output.insert(coord, State::Active);
                        } else {
                            output.insert(coord, State::Inactive);
                        }
                    }
                }
            }
        }
    }

    output
}

fn step_n_cycles_3(input: &HashMap<Coord3, State>, n: usize) -> HashMap<Coord3, State> {
    let mut output = input.clone();

    for _ in 0..n {
        output = step_one_cycle_3(&output);
    }

    output.clone()
}

fn _print_state(data: &HashMap<Coord3, State>) {
    let (x_range, y_range, z_range) = coord_range_3(data);


    for z in z_range {
        println!("z = {}", z);

        for y in y_range.clone() {
            for x in x_range.clone() {
                let value = data.get(&(x, y, z)).unwrap_or(&State::Inactive);

                match value {
                    State::Active => { print!("#"); }
                    State::Inactive => { print!("."); }
                }
            }

            println!();
        }

        println!();
    }
}

#[aoc(day17, part1)]
fn six_cycles_active_count(input: &HashMap<Coord3, State>) -> usize {
    let result = step_n_cycles_3(input, 6);

    result.values().filter(|&&value| {
        match value {
            State::Active => true,
            _ => false
        }
    }).count()
}


fn map_to_hypercube(input: &HashMap<Coord3, State>) -> HashMap<Coord4, State> {
    let mut output = HashMap::new();

    for (&(x,y,z), &state) in input.iter() {
        output.insert((x,y,z,0), state);
    }

    output
}

// count of (Active, Inactive)
fn count_neighboring_states_4(input: &HashMap<Coord4, State>, coord: &Coord4) -> (usize, usize) {
    let &(x0, y0, z0, w0) = coord;

    let mut actives = 0;
    let mut inactives = 0;

    for x in x0-1 ..= x0+1 {
        for y in y0-1 ..= y0+1 {
            for z in z0-1 ..= z0+1 {
                for w in w0-1 ..= w0+1 {
                    if (x, y, z, w) != (x0, y0, z0, w0) {
                        match input.get(&(x, y, z, w)).unwrap_or(&State::Inactive) {
                            State::Inactive => { inactives += 1; }
                            State::Active => { actives += 1; }
                        }
                    }
                }
            }
        }
    }

    (actives, inactives)
}

type CoordRange4 = (RangeInclusive<isize>, RangeInclusive<isize>, RangeInclusive<isize>, RangeInclusive<isize>);

fn coord_range_4(data: &HashMap<Coord4, State>) -> CoordRange4 {
    let mut x_min = None;
    let mut y_min = None;
    let mut z_min = None;
    let mut x_max = None;
    let mut y_max = None;
    let mut z_max = None;
    let mut w_min = None;
    let mut w_max = None;

    let keys: Vec<&Coord4> = data.keys().collect();

    for &(x, y, z, w) in keys {
        if x_min.is_none() || x < x_min.unwrap() {
            x_min = Some(x);
        }

        if y_min.is_none() || y < y_min.unwrap() {
            y_min = Some(y);
        }

        if z_min.is_none() || z < z_min.unwrap() {
            z_min = Some(z);
        }

        if w_min.is_none() || w < w_min.unwrap() {
            w_min = Some(w);
        }

        if x_max.is_none() || x > x_max.unwrap() {
            x_max = Some(x);
        }

        if y_max.is_none() || y > y_max.unwrap() {
            y_max = Some(y);
        }

        if z_max.is_none() || z > z_max.unwrap() {
            z_max = Some(z);
        }

        if w_max.is_none() || w > w_max.unwrap() {
            w_max = Some(w);
        }
    }

    (
        x_min.unwrap() - 1 ..= x_max.unwrap() + 1,
        y_min.unwrap() - 1 ..= y_max.unwrap() + 1,
        z_min.unwrap() - 1 ..= z_max.unwrap() + 1,
        w_min.unwrap() - 1 ..= w_max.unwrap() + 1
    )
}

fn step_one_cycle_4(input: &HashMap<Coord4, State>) -> HashMap<Coord4, State> {
    let mut output = HashMap::new();

    let (x_range, y_range, z_range, w_range) = coord_range_4(input);

    for x in x_range {
        for y in y_range.clone() {
            for z in z_range.clone() {
                for w in w_range.clone() {
                    let coord = (x, y, z, w);

                    let (actives, _inactives) = count_neighboring_states_4(input, &coord);

                    let current_state = input.get(&coord).unwrap_or(&State::Inactive);

                    match current_state {
                        State::Active => {
                            if actives == 2 || actives == 3 {
                                output.insert(coord, State::Active);
                            } else {
                                output.insert(coord, State::Inactive);
                            }
                        },
                        State::Inactive => {
                            if actives == 3 {
                                output.insert(coord, State::Active);
                            } else {
                                output.insert(coord, State::Inactive);
                            }
                        }
                    }
                }
            }
        }
    }

    output
}

fn step_n_cycles_4(input: &HashMap<Coord4, State>, n: usize) -> HashMap<Coord4, State> {
    let mut output = input.clone();

    for _ in 0..n {
        output = step_one_cycle_4(&output);
    }

    output.clone()
}

#[aoc(day17, part2)]
fn hypercube_six_cycles_active_count(input: &HashMap<Coord3, State>) -> usize {
    let hypercube = map_to_hypercube(input);

    let result = step_n_cycles_4(&hypercube, 6);

    result.values().filter(|&&value| {
        match value {
            State::Active => true,
            _ => false
        }
    }).count()
}
