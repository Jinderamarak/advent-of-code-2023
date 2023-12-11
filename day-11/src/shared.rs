use core::panic;

#[derive(Debug, PartialEq, Clone)]
pub struct Image {
    pub w: usize,
    pub h: usize,
    pub data: Vec<Vec<Cell>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    Empty,
    Galaxy,
}

impl Image {
    pub fn parse(data: &str) -> Image {
        let data: Vec<Vec<Cell>> = data
            .lines()
            .map(|l| l.chars().map(Cell::parse).collect())
            .collect();

        Image {
            w: data[0].len(),
            h: data.len(),
            data,
        }
    }

    pub fn empty_rows(&self) -> Vec<usize> {
        let mut rows = Vec::new();
        for y in 0..self.h {
            if self.data[y].iter().all(|c| *c == Cell::Empty) {
                rows.push(y);
            }
        }
        rows
    }

    pub fn empty_cols(&self) -> Vec<usize> {
        let mut cols = Vec::new();
        for x in 0..self.w {
            if self.data.iter().map(|r| r[x]).all(|c| c == Cell::Empty) {
                cols.push(x);
            }
        }
        cols
    }

    pub fn galaxies(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if self.data[y][x] == Cell::Galaxy {
                    result.push((x, y));
                }
            }
        }

        result
    }
}

impl Cell {
    pub fn parse(c: char) -> Cell {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Galaxy,
            _ => panic!("Unknown cell type"),
        }
    }
}

pub fn stretch_point(
    p: (usize, usize),
    rows: &Vec<usize>,
    cols: &Vec<usize>,
    stretcher: usize,
) -> (usize, usize) {
    let (x, y) = p;
    let rows = rows.iter().filter(|n| **n < y).count();
    let cols = cols.iter().filter(|n| **n < x).count();
    (x - cols + cols * stretcher, y - rows + rows * stretcher)
}

pub fn point_distance(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    let (x1, y1) = (p1.0 as isize, p1.1 as isize);
    let (x2, y2) = (p2.0 as isize, p2.1 as isize);

    let dx = (x1 - x2).abs() as usize;
    let dy = (y1 - y2).abs() as usize;
    dx + dy
}
