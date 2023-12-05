use super::parsing::Almanac;

pub fn seeds_to_locations(data: &str) -> Vec<u32> {
    let almanac = Almanac::parse(data);
    almanac
        .seeds
        .iter()
        .map(|seed| seed_to_location(seed, &almanac))
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
