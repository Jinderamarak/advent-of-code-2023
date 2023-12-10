use std::collections::VecDeque;

use super::parsing::{Direction, Tile};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    White,
    Gray,
    Black,
}

pub fn solve(data: &str) -> u32 {
    let map = Tile::parse(data);

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

    find_max_loop(&map, &bfs).try_into().unwrap()
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

fn find_max_loop(map: &Vec<Vec<Tile>>, bfs: &Vec<Vec<(Color, i64)>>) -> i64 {
    let mut maxd = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let d = bfs[y][x].1;
            if d != i64::MAX && d > maxd {
                maxd = d;
            }
        }
    }

    maxd
}
