use aoc_runner_derive::{aoc, aoc_generator};

type Timestamp = usize;
type BusId = usize;

#[derive(Debug, Clone)]
struct Schedule {
    timestamp: Timestamp,
    bus_ids: Vec<BusId>
}

impl Schedule {
    fn next_departure_times(&self) -> Vec<(BusId, Timestamp)> {
        self.bus_ids.iter().map(|&bus_id| {
            (bus_id, (self.timestamp / bus_id + 1) * bus_id)
        }).collect()
    }

    fn next_available_bus_and_time(&self) -> (BusId, Timestamp) {
        let mut departure_times = self.next_departure_times();

        departure_times.sort_by(|(_, ts_a), (_, ts_b)| {
            ts_a.cmp(ts_b)
        });

        departure_times.first().unwrap().clone()
    }
}

#[aoc_generator(day13, part1)]
fn parse_schedule(input: &str) -> Schedule {
    let lines: Vec<&str> = input.lines().collect();

    Schedule {
        timestamp: lines[0].parse().unwrap(),
        bus_ids: lines[1].split(",").filter_map(|val| {
            match val {
                "x" => None,
                _ => val.parse().ok()
            }
        }).collect()
    }
}

#[aoc(day13, part1)]
fn bus_id_times_minutes(schedule: &Schedule) -> usize {
    let next_avail = schedule.next_available_bus_and_time();
    let (bus_id, next_timestamp) = next_avail;

    bus_id * (next_timestamp - schedule.timestamp)
}

#[derive(Debug, Clone)]
struct PositionedSchedule {
    bus_ids_with_offsets: Vec<(usize, BusId)>
}

#[aoc_generator(day13, part2)]
fn parse_schedule_with_positions(input: &str) -> PositionedSchedule {
    let lines: Vec<&str> = input.lines().collect();

    PositionedSchedule {
        bus_ids_with_offsets: lines[1].split(",").enumerate().filter_map(|val| {
            match val {
                (_, "x") => None,
                (i, val) => Some((i, val.parse().unwrap()))
            }
        }).collect()
    }
}

// Got tired and looked up the solution to this one, so it looks a lot like
// https://gist.github.com/jacobchrismarsh/485253dbd42da10ad92d6cc03559cd84#file-day13-rs-L39
#[aoc(day13, part2)]
fn next_timestamp_aligned(schedule: &PositionedSchedule) -> usize {
    let mut buses: Vec<(usize, BusId)> = schedule.bus_ids_with_offsets.clone();
    buses.sort_by_key(|&(_, bus)| bus);

    let mut possible_solution = 0;
    let mut least_common_denominator = 1;
    buses.iter().for_each(|(offset, bus)| {
        while (possible_solution + offset) % bus != 0 {
            possible_solution += least_common_denominator;
        }
        least_common_denominator *= bus;
    });

    possible_solution
}
