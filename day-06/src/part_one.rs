use super::parsing::Race;

pub fn solve(data: &str) -> Vec<u32> {
    Race::parse(data).iter().map(ways_to_beat).collect()
}

fn ways_to_beat(race: &Race) -> u32 {
    (1..race.time)
        .map(|h| Race::traveled_distance(h, race.time))
        .filter(|d| *d > race.distance)
        .count() as u32
}
