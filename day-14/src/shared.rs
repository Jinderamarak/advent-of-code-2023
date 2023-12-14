#[derive(Debug, Clone)]
pub struct Platform {
    pub w: usize,
    pub h: usize,
    pub rows: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Rock,
    Cube,
    Air
}

impl Platform {
    pub fn parse(data: &str) -> Platform {
        let rows: Vec<Vec<Tile>> = data.lines().map(|l| l.chars().map(Tile::parse).collect()).collect();
        Platform {
            w: rows[0].len(),
            h: rows.len(),
            rows,
        }
    }

    pub fn print(&self) {
        println!();
        for row in &self.rows {
            for tile in row {
                tile.print();
            }
            println!();
        }
    }
}

impl Tile {
    pub fn parse(c: char) -> Tile {
        match c {
            'O' => Tile::Rock,
            '#' => Tile::Cube,
            '.' => Tile::Air,
            c => panic!("Unexpected tile '{c}'"),
        }
    }

    pub fn print(&self) {
        match self {
            Tile::Rock => print!("O"),
            Tile::Cube => print!("#"),
            Tile::Air => print!("."),
        }
    }
}
