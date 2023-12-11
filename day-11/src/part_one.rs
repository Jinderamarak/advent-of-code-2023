use std::collections::VecDeque;

use crate::parsing::Cell;

use super::parsing::Image;

pub fn solve(data: &str) -> Vec<u64> {
    let image = stretch(&Image::parse(data));
    let galaxies = image.galaxies();

    let mut distances = Vec::new();
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let start = galaxies[i];
            let end = galaxies[j];

            let dist = do_bfs(start, end, image.w, image.h);
            distances.push(u64::try_from(dist).unwrap());
        }
    }

    distances
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    White,
    Gray,
    Black,
}

fn do_bfs(start: (usize, usize), end: (usize, usize), width: usize, height: usize) -> isize {
    let mut queue = VecDeque::new();
    let mut bfs: Vec<Vec<(Color, isize)>> = (0..height)
        .map(|_| (0..width).map(|_| (Color::White, isize::MAX)).collect())
        .collect();

    bfs[start.1][start.0] = (Color::Gray, 0);
    queue.push_front(start);

    while let Some((ux, uy)) = queue.pop_back() {
        for (vx, vy) in connected_edges(ux, uy, width, height) {
            if bfs[vy][vx].0 == Color::White {
                bfs[vy][vx] = (Color::Gray, bfs[uy][ux].1 + 1);
                queue.push_front((vx, vy));
            }
        }

        bfs[uy][ux] = (Color::Black, bfs[uy][ux].1);
        if ux == end.0 && uy == end.1 {
            return bfs[uy][ux].1;
        }
    }

    isize::MIN
}

fn connected_edges(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    if x > 0 {
        points.push((x - 1, y));
    }
    if y > 0 {
        points.push((x, y - 1));
    }
    if x < w - 1 {
        points.push((x + 1, y));
    }
    if y < h - 1 {
        points.push((x, y + 1));
    }

    points
}

fn stretch(img: &Image) -> Image {
    let rows = img.empty_rows();
    let cols = img.empty_cols();

    let mut data: Vec<Vec<Cell>> = Vec::new();
    for y in 0..img.h {
        let mut row = Vec::new();
        for x in 0..img.w {
            if cols.contains(&x) {
                row.push(img.data[y][x]);
            }
            row.push(img.data[y][x]);
        }

        if rows.contains(&y) {
            data.push(row.clone());
        }
        data.push(row);
    }

    Image {
        w: data[0].len(),
        h: data.len(),
        data,
    }
}
