use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize)
}

impl FromStr for Instr {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Instr, Self::Err> {
        let chars = input.chars().collect::<Vec<char>>();
        match &chars[..] {
            ['N', rest @ ..] => Ok(Instr::N(rest.into_iter().collect::<String>().parse().unwrap())),
            ['S', rest @ ..] => Ok(Instr::S(rest.into_iter().collect::<String>().parse().unwrap())),
            ['E', rest @ ..] => Ok(Instr::E(rest.into_iter().collect::<String>().parse().unwrap())),
            ['W', rest @ ..] => Ok(Instr::W(rest.into_iter().collect::<String>().parse().unwrap())),
            ['F', rest @ ..] => Ok(Instr::F(rest.into_iter().collect::<String>().parse().unwrap())),
            ['L', rest @ ..] => Ok(Instr::L(rest.into_iter().collect::<String>().parse().unwrap())),
            ['R', rest @ ..] => Ok(Instr::R(rest.into_iter().collect::<String>().parse().unwrap())),
            _ => Err("unparseable instruction")
        }
    }
}

#[aoc_generator(day12)]
fn parse_instructions(input: &str) -> Vec<Instr> {
    input.lines().map(|line| line.parse::<Instr>().unwrap()).collect()
}

trait Navigable {
    fn execute(&self, instrs: &[Instr]) -> Self;

    fn manhattan_dist(&self, other: &Self) -> usize;
}

#[derive(Debug, Copy, Clone)]
struct Ship {
    pos_x: isize,
    pos_y: isize,
    dir: Dir,
}

impl Ship {
    fn new(dir: Dir) -> Ship {
        Ship {
            pos_x: 0,
            pos_y: 0,
            dir
        }
    }

    fn turn_ship(&self, instr: &Instr) -> Ship {
        let ship = self.clone();

        let dirs = vec![Dir::N, Dir::E, Dir::S, Dir::W];

        match instr {
            Instr::L(angle) => {
                let num_rotations = angle / 90;

                let (index, _) = dirs.iter().find_position(|&&d| d == ship.dir).unwrap();
                let new_index = (index as isize - num_rotations).rem_euclid(4) as usize;

                Ship { dir: dirs[new_index], ..ship }
            },
            Instr::R(angle) => {
                let num_rotations = angle / 90;

                let (index, _) = dirs.iter().find_position(|&&d| d == ship.dir).unwrap();
                let new_index = (index as isize + num_rotations).rem_euclid(4) as usize;

                Ship { dir: dirs[new_index], ..ship }
            },
            _ => unreachable!("unexpected turn angle")
        }
    }
}

impl Navigable for Ship {
    fn execute(&self, instrs: &[Instr]) -> Ship {
        instrs.iter().fold(self.clone(), |ship, instr| {
            match instr {
                Instr::N(dist) => Ship{ pos_y: ship.pos_y + dist, ..ship },
                Instr::S(dist) => Ship{ pos_y: ship.pos_y - dist, ..ship },
                Instr::E(dist) => Ship{ pos_x: ship.pos_x + dist, ..ship },
                Instr::W(dist) => Ship{ pos_x: ship.pos_x - dist, ..ship },
                Instr::F(dist) => match ship.dir {
                    Dir::N => Ship{ pos_y: ship.pos_y + dist, ..ship },
                    Dir::S => Ship{ pos_y: ship.pos_y - dist, ..ship },
                    Dir::E => Ship{ pos_x: ship.pos_x + dist, ..ship },
                    Dir::W => Ship{ pos_x: ship.pos_x - dist, ..ship }
                }
                turn => ship.turn_ship(turn)
            }
        })
    }

    fn manhattan_dist(&self, other_ship: &Ship) -> usize {
        (other_ship.pos_y - self.pos_y).abs() as usize +
            (other_ship.pos_x - self.pos_x).abs() as usize
    }
}

#[aoc(day12, part1)]
fn manhattan_dist(instrs: &Vec<Instr>) -> usize {
    let ship = Ship::new(Dir::E);

    let new_ship = ship.execute(instrs);

    new_ship.manhattan_dist(&ship)
}

#[derive(Debug, Copy, Clone)]
struct Waypoint {
    rel_x: isize,
    rel_y: isize,
}

impl Waypoint {
    fn move_north(&self, dist: &isize) -> Waypoint {
        Waypoint{ rel_y: self.rel_y + dist, ..*self }
    }

    fn move_south(&self, dist: &isize) -> Waypoint {
        Waypoint{ rel_y: self.rel_y - dist, ..*self }
    }

    fn move_east(&self, dist: &isize) -> Waypoint {
        Waypoint{ rel_x: self.rel_x + dist, ..*self }
    }

    fn move_west(&self, dist: &isize) -> Waypoint {
        Waypoint{ rel_x: self.rel_x - dist, ..*self }
    }

    fn turn_waypoint_clockwise(&self, num_rotations: &isize) -> Waypoint {
        match num_rotations {
            1 => Waypoint{ rel_y: -self.rel_x, rel_x: self.rel_y },
            2 => Waypoint{ rel_y: -self.rel_y, rel_x: -self.rel_x },
            3 => Waypoint{ rel_y: self.rel_x, rel_x: -self.rel_y },
            _ => unreachable!("unreasonable num rotations")
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct WaypointShip {
    pos_x: isize,
    pos_y: isize,
    waypoint: Waypoint
}

impl WaypointShip {
    fn new(init_waypoint_x: isize, init_waypoint_y: isize) -> WaypointShip {
        WaypointShip{
            pos_x: 0,
            pos_y: 0,
            waypoint: Waypoint{ rel_x: init_waypoint_x, rel_y: init_waypoint_y }
        }
    }

    fn move_waypoint_north(&self, dist: &isize) -> WaypointShip {
        WaypointShip{
            waypoint: self.waypoint.move_north(dist),
            ..*self
        }
    }

    fn move_waypoint_south(&self, dist: &isize) -> WaypointShip {
        WaypointShip{
            waypoint: self.waypoint.move_south(dist),
            ..*self
        }
    }

    fn move_waypoint_east(&self, dist: &isize) -> WaypointShip {
        WaypointShip{
            waypoint: self.waypoint.move_east(dist),
            ..*self
        }
    }

    fn move_waypoint_west(&self, dist: &isize) -> WaypointShip {
        WaypointShip{
            waypoint: self.waypoint.move_west(dist),
            ..*self
        }
    }

    fn move_to_waypoint(&self, times: &isize) -> WaypointShip {
        let mut ship = self.clone();

        for _ in 0..*times {
            ship.pos_x += ship.waypoint.rel_x;
            ship.pos_y += ship.waypoint.rel_y;
        }

        ship
    }

    fn turn_waypoint(&self, instr: &Instr) -> WaypointShip {
        let ship = self.clone();

        match instr {
            Instr::L(angle) => {
                let num_rotations = (360 - angle) / 90;

                WaypointShip {
                    waypoint: self.waypoint.turn_waypoint_clockwise(&num_rotations),
                    ..ship
                }
            },
            Instr::R(angle) => {
                let num_rotations = angle / 90;

                WaypointShip {
                    waypoint: self.waypoint.turn_waypoint_clockwise(&num_rotations),
                    ..ship
                }
            },
            _ => unreachable!("unexpected turn angle")
        }
    }
}

impl Navigable for WaypointShip {
    fn execute(&self, instrs: &[Instr]) -> WaypointShip {
        instrs.iter().fold(self.clone(), |ship, instr| {
            match instr {
                Instr::N(dist) => ship.move_waypoint_north(dist),
                Instr::S(dist) => ship.move_waypoint_south(dist),
                Instr::E(dist) => ship.move_waypoint_east(dist),
                Instr::W(dist) => ship.move_waypoint_west(dist),
                Instr::F(times) => ship.move_to_waypoint(times),
                turn => ship.turn_waypoint(turn)
            }
        })
    }

    fn manhattan_dist(&self, other_ship: &WaypointShip) -> usize {
        (other_ship.pos_y - self.pos_y).abs() as usize +
            (other_ship.pos_x - self.pos_x).abs() as usize
    }
}



#[aoc(day12, part2)]
fn waypoint_manhattan_dist(instrs: &Vec<Instr>) -> usize {
    let ship = WaypointShip::new(10, 1);

    let new_ship = ship.execute(instrs);

    new_ship.manhattan_dist(&ship)
}
