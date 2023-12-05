use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::parsing::Almanac;

pub fn seeds_to_locations(data: &str) -> Vec<u32> {
    let almanac = Almanac::parse(data);
    let ranges = iter_pairs(almanac.seeds.iter());

    let seeds: Vec<_> = ranges
        .iter()
        .map(|(s, l)| **s..(**s + **l))
        .flatten()
        .collect();

    seeds
        .par_iter()
        .map(|seed| seed_to_location(&seed, &almanac))
        .collect()
}

fn seed_to_location(location: &u32, almanac: &Almanac) -> u32 {
    let mut system = "seed";
    let mut location = *location;
    while system != "location" {
        (system, location) = almanac.map(system, location);
    }

    location
}

fn iter_pairs<T>(mut values: impl Iterator<Item = T>) -> Vec<(T, T)> {
    let mut result = Vec::new();
    while let Some(i1) = values.next() {
        let i2 = values.next().unwrap();
        result.push((i1, i2));
    }

    result
}
