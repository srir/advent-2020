use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied
}

type State = Vec<Vec<Seat>>;

#[aoc_generator(day11)]
fn parse_initial_state(input: &str) -> State {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                'L' => Seat::Empty,
                '.' => Seat::Floor,
                '#' => Seat::Occupied,
                _ => unreachable!("Unexpected seat character")
            }
        }).collect()
    }).collect()
}

fn step_state(state: &State) -> State {
    let mut new_state = state.clone();

    let rows = new_state.len() as i32;
    let cols  = new_state.first().unwrap().len() as i32;

    state.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, seat)| {
            let coord_diffs = vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1)
            ];

            let num_adjacent_occupied_seats: usize = coord_diffs.iter().map(|(y_inc, x_inc)| {
                let new_y = y as i32 + y_inc;
                let new_x = x as i32 + x_inc;

                if 0 <= new_y && new_y < rows
                    && 0 <= new_x && new_x < cols
                    && state[new_y as usize][new_x as usize] == Seat::Occupied {
                    1
                } else {
                    0
                }
            }).sum();

            match seat {
                Seat::Occupied if num_adjacent_occupied_seats >= 4 => {
                    new_state[y][x] = Seat::Empty;
                },
                Seat::Empty if num_adjacent_occupied_seats == 0 => {
                    new_state[y][x] = Seat::Occupied;
                },
                _ => ()
            };
        });
    });

    new_state
}

fn step_state_2(state: &State) -> State {
    let mut new_state = state.clone();

    let rows = new_state.len() as i32;
    let cols  = new_state.first().unwrap().len() as i32;

    state.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, seat)| {
            let coord_diffs = vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1)
            ];

            let num_adjacent_occupied_seats: usize = coord_diffs.iter().map(|(y_inc, x_inc)| {
                let mut new_y = y as i32 + y_inc;
                let mut new_x = x as i32 + x_inc;

                while 0 <= new_y && new_y < rows
                    && 0 <= new_x && new_x < cols
                    && state[new_y as usize][new_x as usize] == Seat::Floor {
                    new_y += y_inc;
                    new_x += x_inc;
                }

                if 0 <= new_y && new_y < rows
                    && 0 <= new_x && new_x < cols
                    && state[new_y as usize][new_x as usize] == Seat::Occupied {
                    1
                } else {
                    0
                }
            }).sum();

            match seat {
                Seat::Occupied if num_adjacent_occupied_seats >= 5 => {
                    new_state[y][x] = Seat::Empty;
                },
                Seat::Empty if num_adjacent_occupied_seats == 0 => {
                    new_state[y][x] = Seat::Occupied;
                },
                _ => ()
            };
        });
    });

    new_state
}

fn _count_occupied_seats(state: &State) -> usize {
    state.iter().map(|row| {
        row.iter().map(|seat| {
            match seat {
                Seat::Occupied => 1,
                _ => 0
            }
        }).sum::<usize>()
    }).sum::<usize>()
}

#[aoc(day11, part1)]
fn count_occupied_seats_after_settled(state: &State) -> usize {
    let mut new_state = state.clone();
    let mut mutated_state = step_state(state);

    while new_state != mutated_state {
        new_state = mutated_state;
        mutated_state = step_state(&new_state);
    }

    _count_occupied_seats(&new_state)
}

#[aoc(day11, part2)]
fn count_occupied_seats_after_settled_2(state: &State) -> usize {
    let mut new_state = state.clone();
    let mut mutated_state = step_state_2(state);

    while new_state != mutated_state {
        new_state = mutated_state;
        mutated_state = step_state_2(&new_state);
    }

    _count_occupied_seats(&new_state)
}
