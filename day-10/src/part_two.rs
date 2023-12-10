use std::collections::VecDeque;

use super::parsing::{Direction, Tile};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    White,
    Gray,
    Black,
}

pub fn solve(data: &str) -> u64 {
    let mut map = Tile::parse(data);

    let mut bfs: Vec<Vec<(Color, i64)>> = map
        .iter()
        .map(|r| r.iter().map(|_| (Color::White, i64::MAX)).collect())
        .collect();
    let mut queue = VecDeque::new();

    let (sx, sy) = Tile::find_start(&map);
    bfs[sy][sx] = (Color::Gray, 0);
    queue.push_front((sx, sy));

    while let Some((ux, uy)) = queue.pop_back() {
        for (vx, vy) in connected_edges(&map, ux, uy) {
            if bfs[vy][vx].0 == Color::White {
                bfs[vy][vx] = (Color::Gray, bfs[uy][ux].1 + 1);
                queue.push_front((vx, vy));
            }
        }

        bfs[uy][ux] = (Color::Black, bfs[uy][ux].1);
    }

    map[sy][sx] = center_replacement(&map, sx, sy);

    let graph: Vec<Vec<bool>> = bfs
        .iter()
        .map(|r| r.iter().map(|c| c.0 == Color::Black).collect())
        .collect();

    let mut result: Vec<Vec<i32>> = map.iter().map(|r| r.iter().map(|_| -1).collect()).collect();

    //println!();
    let mut area = 0;
    let mut last_corner = None;
    for y in 0..result.len() {
        //print!("(");
        let mut count = 0;
        for x in 0..result[y].len() {
            if graph[y][x] {
                if is_corner(&map[y][x]) {
                    let now = corner_dir(&map[y][x]);
                    if let Some(dir) = last_corner {
                        if dir != now {
                            count += 1;
                        }
                        last_corner = None;
                        //print!("<");
                    } else {
                        last_corner = Some(now);
                        //print!(">");
                    }
                } else if is_vertical(&map[y][x]) {
                    count += 1;
                    //print!("|");
                } else {
                    //print!("-");
                }
            } else {
                if count % 2 == 1 {
                    area += 1;
                    //print!("X");
                } else {
                    //print!(" ");
                }
            }

            result[y][x] = count;
        }
        //println!(")");
    }

    //println!();
    for y in 0..result.len() {
        for x in 0..result[y].len() {
            let x = result[y][x];
            if x < 10 {
                //print!(" {x} ");
            } else {
                //print!("{x} ");
            }
        }
        //println!();
    }

    area
}

fn connected_edges(map: &Vec<Vec<Tile>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    if x > 0 {
        edges.push((x - 1, y, Direction::West));
    }
    if x < map[0].len() - 1 {
        edges.push((x + 1, y, Direction::East));
    }
    if y > 0 {
        edges.push((x, y - 1, Direction::North));
    }
    if y < map.len() - 1 {
        edges.push((x, y + 1, Direction::South));
    }

    edges
        .iter()
        .filter(|(nx, ny, d)| map[y][x].connects_to(&map[*ny][*nx], *d))
        .map(|(x, y, _)| (*x, *y))
        .collect()
}

fn is_horizontal(tile: &Tile) -> bool {
    match tile {
        Tile::Pipe(Direction::West, Direction::East) => true,
        Tile::Pipe(Direction::East, Direction::West) => true,
        _ => false,
    }
}

fn is_vertical(tile: &Tile) -> bool {
    match tile {
        Tile::Pipe(Direction::North, Direction::South) => true,
        Tile::Pipe(Direction::South, Direction::North) => true,
        _ => false,
    }
}

fn is_harry_potter(before: &Tile, now: &Tile) -> bool {
    match (before, now) {
        (
            Tile::Pipe(Direction::North, Direction::East),
            Tile::Pipe(Direction::South, Direction::West),
        ) => true,
        (
            Tile::Pipe(Direction::South, Direction::East),
            Tile::Pipe(Direction::North, Direction::West),
        ) => true,
        _ => false,
    }
}

fn is_corner(tile: &Tile) -> bool {
    match tile {
        Tile::Pipe(a, b) => a != b && a.opposite() != *b,
        _ => false,
    }
}

fn corner_dir(tile: &Tile) -> Direction {
    match tile {
        Tile::Pipe(a, _) => *a,
        _ => Direction::North,
    }
}

fn is_counted(tile: &Tile) -> bool {
    match tile {
        Tile::Pipe(Direction::North, Direction::South) => true,
        Tile::Pipe(Direction::North, Direction::East) => true,
        Tile::Pipe(Direction::South, Direction::East) => true,
        _ => false,
    }
}

fn center_replacement(map: &Vec<Vec<Tile>>, sx: usize, sy: usize) -> Tile {
    let top = if sy == 0 {
        map[sy][sx]
    } else {
        map[sy - 1][sx]
    };
    let right = if sx == map[0].len() - 1 {
        map[sy][sx]
    } else {
        map[sy][sx + 1]
    };
    let bottom = if sy == map.len() - 1 {
        map[sy][sx]
    } else {
        map[sy + 1][sx]
    };
    let left = if sx == 0 {
        map[sy][sx]
    } else {
        map[sy][sx - 1]
    };

    let mut vertical = Direction::North;
    let mut horizontal = Direction::East;

    if matches!(top, Tile::Pipe(Direction::South, _))
        || matches!(top, Tile::Pipe(_, Direction::South))
    {
        vertical = Direction::North;
    }
    if matches!(right, Tile::Pipe(Direction::West, _))
        || matches!(right, Tile::Pipe(_, Direction::West))
    {
        horizontal = Direction::East;
    }
    if matches!(bottom, Tile::Pipe(Direction::North, _))
        || matches!(bottom, Tile::Pipe(_, Direction::North))
    {
        vertical = Direction::South;
    }
    if matches!(left, Tile::Pipe(Direction::East, _))
        || matches!(left, Tile::Pipe(_, Direction::East))
    {
        horizontal = Direction::West;
    }

    Tile::Pipe(vertical, horizontal)
}
