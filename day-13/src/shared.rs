#[derive(Debug, Clone)]
pub struct Pattern {
    pub w: usize,
    pub h: usize,
    pub rows: Vec<Vec<char>>,
}

pub enum Reflection {
    Horizontal(u32),
    Vertical(u32),
}

pub type Detector = dyn Fn(&Pattern, usize) -> bool;

impl Pattern {
    pub fn parse(data: &str) -> Pattern {
        let rows = data
            .lines()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>();

        Pattern {
            w: rows[0].len(),
            h: rows.len(),
            rows,
        }
    }

    pub fn rotate(&self) -> Pattern {
        let mut rows: Vec<Vec<char>> = (0..self.w)
            .map(|_| (0..self.h).map(|_| 'x').collect())
            .collect();

        for y in 0..self.h {
            for x in 0..self.w {
                rows[x][y] = self.rows[y][x];
            }
        }

        Pattern {
            w: self.h,
            h: self.w,
            rows,
        }
    }

    pub fn find_reflection(&self, detector: &Detector) -> Reflection {
        if let Some(a) = self.find_horizontal(detector) {
            return Reflection::Horizontal(a);
        }

        let rotated = self.rotate();
        Reflection::Vertical(rotated.find_horizontal(detector).unwrap())
    }

    fn find_horizontal(&self, detector: &Detector) -> Option<u32> {
        for y in 1..self.h {
            if detector(self, y) {
                return Some(y.try_into().unwrap());
            }
        }

        None
    }
}

impl Reflection {
    pub fn summarize(self) -> u32 {
        match self {
            Self::Horizontal(x) => x * 100,
            Self::Vertical(x) => x,
        }
    }
}
