use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, HashMap};

type TileId = usize;
type TileRow = Vec<char>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Tile {
    id: TileId,
    data: Vec<TileRow>
}

impl Tile {
    fn top(&self) -> TileRow {
        self.data[0].clone()
    }

    fn bottom(&self) -> TileRow {
        self.data[self.data.len() - 1].iter().rev().cloned().collect()
    }

    fn left(&self) -> TileRow {
        self.data.iter().rev().map(|row| row[0]).collect()
    }

    fn right(&self) -> TileRow {
        self.data.iter().map(|row| row[row.len()-1]).collect()
    }

    fn sides(&self) -> Vec<TileRow> {
        vec![self.top(), self.bottom(), self.left(), self.right()]
    }
}

#[aoc_generator(day20)]
fn parse_puzzle(input: &str) -> Vec<Tile> {
    input.split("\n\n").map(|tile_data| {
        let lines = tile_data.lines().collect::<Vec<&str>>();

        let id_line = lines[0];
        let id = id_line[5..id_line.len()-1].parse().unwrap();

        let data = lines[1..].iter().map(|line| {
            line.chars().collect()
        }).collect();

        Tile { id, data }
    }).collect()
}

struct Puzzle {
    tiles: HashSet<Tile>
}

impl Puzzle {
    fn new(tiles: &Vec<Tile>) -> Self {
        let tiles: HashSet<Tile> = tiles.into_iter().cloned().collect();

        Puzzle { tiles }
    }

    fn find_corner_tiles(&self) -> HashSet<TileId> {
        let tiles = &self.tiles;
        let mut matches = HashMap::<TileId, HashSet<TileId>>::new();
        let mut side_to_tile = HashMap::<TileRow, TileId>::new();

        for tile in tiles.iter() {
            let mut find_matches = |side| {
                if let Some(prev) = side_to_tile.insert(side, tile.id) {
                    matches.entry(tile.id).and_modify(|x| {x.insert(prev);}).or_insert([prev].iter().cloned().collect());
                    matches.entry(prev).and_modify(|x| {x.insert(tile.id);}).or_insert([tile.id].iter().cloned().collect());
                }
            };

            for side in tile.sides() {
                find_matches(side.iter().rev().cloned().collect());
                find_matches(side);
            }
        }

        matches.iter().filter(|(_, other_tiles)| {
            other_tiles.len() == 2
        }).map(|(tile_id, _)| tile_id).cloned().collect()
    }
}

#[aoc(day20, part1)]
fn corner_tile_id_product(input: &Vec<Tile>) -> usize {
    Puzzle::new(&input).find_corner_tiles().iter().product()
}
