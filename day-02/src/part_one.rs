use super::parsing::{Game, Cubes};

const AVAILABLE_CUBES: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

pub fn get_possible_ids(data: &str) -> Vec<u32> {
    data.lines()
        .map(Game::parse)
        .filter(|game| {
            game.peeks
                .iter()
                .all(|peek| peek.fits_within(&AVAILABLE_CUBES))
        })
        .map(|game| game.id)
        .collect()
}