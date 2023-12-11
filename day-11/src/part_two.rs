use super::parsing::Image;

const STRETCH_MULTIPLIER: usize = 1_000_000;

pub fn solve(data: &str) -> Vec<u64> {
    let image = Image::parse(data);
    let rows = image.empty_rows();
    let cols = image.empty_cols();
    let galaxies = image
        .galaxies()
        .iter()
        .map(|p| stretch_point(*p, &rows, &cols))
        .map(|p| (p.0 as i64, p.1 as i64))
        .collect::<Vec<(i64, i64)>>();

    let mut distances = Vec::new();
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let start = galaxies[i];
            let end = galaxies[j];

            let dx = (start.0 - end.0).abs() as u64;
            let dy = (start.1 - end.1).abs() as u64;
            distances.push(dx + dy)
        }
    }

    distances
}

fn stretch_point(p: (usize, usize), rows: &Vec<usize>, cols: &Vec<usize>) -> (usize, usize) {
    let (x, y) = p;
    let rows = rows.iter().filter(|n| **n < y).count();
    let cols = cols.iter().filter(|n| **n < x).count();
    (
        x - cols + cols * STRETCH_MULTIPLIER,
        y - rows + rows * STRETCH_MULTIPLIER,
    )
}
