use super::shared::{point_distance, stretch_point, Image};

const STRETCH_MULTIPLIER: usize = 1_000_000;

pub fn solve(data: &str) -> Vec<usize> {
    let image = Image::parse(data);
    let rows = image.empty_rows();
    let cols = image.empty_cols();

    let galaxies = image
        .galaxies()
        .iter()
        .map(|p| stretch_point(*p, &rows, &cols, STRETCH_MULTIPLIER))
        .collect::<Vec<(usize, usize)>>();

    let mut distances = Vec::new();
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            distances.push(point_distance(&galaxies[i], &galaxies[j]));
        }
    }

    distances
}
