use core::panic;

use self::Direction::{East, North, South, West};
use self::Tile::{Ground, Pipe, Start};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    Start,
    Ground,
    Pipe(Direction, Direction),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Tile {
    pub fn parse(data: &str) -> Vec<Vec<Tile>> {
        data.lines()
            .map(|l| l.chars().map(char_to_tile).collect())
            .collect()
    }

    pub fn find_start(map: &Vec<Vec<Tile>>) -> (usize, usize) {
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == Start {
                    return (x, y);
                }
            }
        }

        panic!("Map is missing start");
    }

    pub fn connects_to(&self, other: &Tile, dir: Direction) -> bool {
        match (self, other, dir) {
            (Ground, _, _) => false,
            (Start, Pipe(a, b), c) => a.opposite() == c || b.opposite() == c,
            (Pipe(a, b), Pipe(c, d), x) => {
                (*a == x || *b == x) && (*c == x.opposite() || *d == x.opposite())
            }
            (a, b, c) => b.connects_to(a, c.opposite()),
        }
    }
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

fn char_to_tile(c: char) -> Tile {
    match c {
        '|' => Pipe(North, South),
        '-' => Pipe(West, East),
        'L' => Pipe(North, East),
        'J' => Pipe(North, West),
        '7' => Pipe(South, West),
        'F' => Pipe(South, East),
        '.' => Ground,
        'S' => Start,
        _ => panic!("Unknown tile {}", c),
    }
}
