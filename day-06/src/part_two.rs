use super::parsing::Race;

pub fn solve(data: &str) -> u32 {
    let race = Race::parse_kerning(data);

    //  This is way too fast with Rust, even without rayon
    (1..race.time)
        .map(|h| Race::traveled_distance(h, race.time))
        .filter(|d| *d > race.distance)
        .count() as u32
}
