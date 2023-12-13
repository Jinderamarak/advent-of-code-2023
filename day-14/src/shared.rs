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

impl Pattern {
    pub fn parse(data: &str) -> Pattern {
        todo!();
    }
}
