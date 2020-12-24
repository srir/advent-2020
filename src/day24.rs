use aoc_runner_derive::{aoc, aoc_generator};
extern crate pom;
use pom::{
    parser::{ Parser, list, seq, empty }
};
use std::collections::HashMap;
use std::ops::RangeInclusive;

enum Instr {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast
}

fn instr_parser<'a>() -> Parser<'a, u8, Instr> {
    seq(b"se").map(|_| Instr::Southeast)
        | seq(b"sw").map(|_| Instr::Southwest)
        | seq(b"ne").map(|_| Instr::Northeast)
        | seq(b"nw").map(|_| Instr::Northwest)
        | seq(b"w").map(|_| Instr::West)
        | seq(b"e").map(|_| Instr::East)
}

fn parser<'a>() -> Parser<'a, u8, Vec<Instr>> {
    list(instr_parser(), empty())
}

fn parse_line(line: &str) -> Vec<Instr> {
    parser().parse(line.as_bytes()).expect("failed to parse")
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Vec<Vec<Instr>> {
    input.lines().map(|line| {
        parse_line(line)
    }).collect()
}

type TileMap = HashMap<(isize, isize, isize), bool>;

struct TileFloor {
    tiles: TileMap,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>,
}

impl TileFloor {
    fn new() -> TileFloor {
        TileFloor {
            tiles: HashMap::new(),
            x_range: 0..=0,
            y_range: 0..=0,
            z_range: 0..=0
        }
    }

    fn toggle(&mut self, (x, y, z): (isize, isize, isize)) {
        if x < *self.x_range.start() {
            self.x_range = x..=*self.x_range.end();
        }

        if x > *self.x_range.end() {
            self.x_range = *self.x_range.start()..=x;
        }

        if y < *self.y_range.start() {
            self.y_range = y..=*self.y_range.end();
        }

        if y > *self.y_range.end() {
            self.y_range = *self.y_range.start()..=y;
        }

        if z < *self.z_range.start() {
            self.z_range = z..=*self.z_range.end();
        }

        if z > *self.z_range.end() {
            self.z_range = *self.z_range.start()..=z;
        }

        self.tiles.entry((x, y, z)).and_modify(|b| *b = !*b).or_insert(true);
    }

    fn flip_tile(&mut self, instrs: &Vec<Instr>) {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        for instr in instrs {
            match instr {
                Instr::East => { x += 1; y -= 1; },
                Instr::West => { x -= 1; y += 1; },
                Instr::Northeast => { x += 1; z -= 1 }
                Instr::Northwest => { y += 1; z -= 1 }
                Instr::Southwest => { x -= 1; z += 1 }
                Instr::Southeast => { y -= 1; z += 1 }
            }
        }

        self.toggle((x, y, z));
    }

    fn flip_tiles(&mut self, tiles: &Vec<Vec<Instr>>) {
        for instrs in tiles {
            self.flip_tile(instrs)
        }
    }

    fn count_black_tiles(&self) -> usize {
        self.tiles.values().filter(|&&v| v).count()
    }

    fn count_adjacent_black_tiles(tiles: &TileMap, (x, y, z): (isize, isize, isize)) -> usize {
        let coord_sets = vec![
            (x+1, y, z-1),
            (x+1, y-1, z),
            (x, y-1, z+1),
            (x-1, y, z+1),
            (x-1, y+1, z),
            (x, y+1, z-1),
        ];

        coord_sets.iter().map(|coords| {
            tiles.get(coords).unwrap_or(&false)
        }).filter(|&&v| v).count()
    }

    fn step(&mut self) {
        let old_tiles = self.tiles.clone();

        for z in (self.z_range.start() - 1)..=(self.z_range.end() + 1) {
            for y in (self.y_range.start() - 1)..=(self.y_range.end() + 1) {
                for x in (self.x_range.start() - 1)..=(self.x_range.end() + 1) {
                    let coords = (x,y,z);
                    let adjacent_black = TileFloor::count_adjacent_black_tiles(&old_tiles, coords);

                    let current = *old_tiles.get(&coords).unwrap_or(&false);

                    match (current, adjacent_black) {
                        (true, c) if c == 0 || c > 2 => {
                            self.toggle(coords);
                        },
                        (false, 2) => {
                            self.toggle(coords);
                        }
                        _ => ()
                    }
                }
            }
        }
    }
}

#[aoc(day24, part1)]
fn count_black_tiles(input: &Vec<Vec<Instr>>) -> usize {
    let mut floor = TileFloor::new();
    floor.flip_tiles(&input);

    floor.count_black_tiles()
}


#[aoc(day24, part2)]
fn simulate_and_count(input: &Vec<Vec<Instr>>) -> usize {
    let mut floor = TileFloor::new();
    floor.flip_tiles(&input);

    for _ in 0..100 {
        floor.step();
    }

    floor.count_black_tiles()
}
