#[derive(Debug, Clone)]
pub struct Pattern {
    pub w: usize,
    pub h: usize,
    pub rows: Vec<Vec<char>>,
}

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
}
